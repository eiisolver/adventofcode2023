package aoc2023;

import java.io.PrintStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Arrays;

public class Day24 {
   private static final long F = 1000000000000L;
   static Line[] lines;
   static P2[] points;
   static double[] speeds;
   static double avgSpeed;
   static Line fScratch = new Line(1, 1, 1, 1);

   static class P2 implements Comparable<P2> {
      int id;
      double x;
      double t;

      @Override
      public int compareTo(P2 p) {
         return (p.x > x) ? -1 : 1;
      }
   }

   static class Line {
      double x, y, vx, vy;
      double a, b; // y = a*x+b
      long xl, yl, zl, vxl, vyl, vzl;

      public Line(double x, double y, double vx, double vy) {
         set(x, y, vx, vy);
      }

      public void set(double x, double y, double vx, double vy) {
         this.x = x;
         this.y = y;
         this.vx = vx;
         this.vy = vy;
         a = vy / vx;
         b = this.y - this.x * a;
      }

      public double xIntersect(Line f2) {
         return (f2.b - b) / (a - f2.a);
      }

      public boolean isInFuture(double x0) {
         return (x0 > x) == (vx > 0);
      }

      public double timeTo(double x0) {
         return (x0 - x) / vx;
      }

      @Override
      public String toString() {
         return "Line [x=" + x + ", y=" + y + ", vx=" + vx + ", vy=" + vy + ", a=" + a + ", b=" + b + "]";
      }

   }

   private static class SlopeResult implements Comparable<SlopeResult>, Cloneable {
      double y0;
      double y1;
      int lineMisses;
      int nrDescending;
      double mse;
      double avgSpeed;

      @Override
      public int compareTo(SlopeResult r) {
         if (lineMisses != r.lineMisses) {
            return r.lineMisses - lineMisses;
         }
         if (nrDescending != r.nrDescending) {
            return nrDescending - r.nrDescending;
         }
         return (int) Math.signum(r.mse - mse);
      }

      @Override
      protected Object clone() throws CloneNotSupportedException {
         return super.clone();
      }

      @Override
      public String toString() {
         return "SlopeResult [y0=" + y0 + ", y1=" + y1 + ", lineMisses=" + lineMisses + ", nrDescending=" + nrDescending
               + ", mse=" + mse + ", avgSpeed=" + avgSpeed + "]";
      }
   }

   public static double meanSquaredError(double[] values) {
      double avg = Arrays.stream(values).average().orElse(Double.NaN);
      avgSpeed = avg;
      double mse = 0;
      for (var v : values) {
         mse += (v - avg) * (v - avg);
      }
      return mse;
   }

   /**
    * Draw a line from left to right in a box, check:
    * - how many lines we intersect (t>0) -> all line misses are recorded in lineMisses
    * - for each intersection we calculate the X coordinate and time (of the hail) when it reaches the intersection
    *   when sorted, ideally all X and times are in the same order (in my puzzle, the rock's vx is negative,
    *   so must be descending)
    * - if all intersections are in a plausible order: calculate the time points between the intersection,
    *   then the x-speed of the rock is delta x / time. This speed should ideally be equal between all intersectinns.
    *   Calculate the mean squared error of the speeds.
    */
   public static void evalSlope(double y0, double y1, SlopeResult result) {
      fScratch.set(0, y0, 2000, y1);
      result.lineMisses = 0;
      result.nrDescending = 0;
      result.mse = 0;
      result.y0 = y0;
      result.y1 = y1;
      for (int i = 0; i < lines.length; ++i) {
         var f2 = lines[i];
         double x = fScratch.xIntersect(f2);
         points[i].x = x;
         points[i].t = f2.timeTo(x);
         points[i].id = i;
         if (points[i].t < 0) {
            ++result.lineMisses;
         }
      }
      if (result.lineMisses == 0) {
         Arrays.sort(points);
         for (int j = 0; j < points.length; ++j) {
            var p = points[j];
            if (j > 0 && p.t <= points[j - 1].t) {
               ++result.nrDescending;
            }
         }
         if (result.nrDescending == points.length - 1) {
            for (int i = 0; i < points.length; ++i) {
               var p = points[i];
               if (i > 0) {
                  var dx = p.x - points[i - 1].x;
                  var dt = p.t - points[i - 1].t;
                  speeds[i - 1] = dx / dt;
               }
            }
            result.mse = meanSquaredError(speeds);
            result.avgSpeed = avgSpeed;
         }
      }
   }

   static void toView(Line bestF) throws Exception {
      PrintStream out = new PrintStream("view.txt");
      out.println("STATE");
      out.printf("type:arrow,x1:%d,y1:%d,x2:%d,y2:%d,color:%s\n", 0, (int) bestF.y, 2000, (int) (bestF.y + bestF.vy),
            "black");
      for (var f2 : lines) {
         var x = f2.x;
         var y = f2.y;
         var vx = f2.vx;
         var vy = f2.vy;
         var x2 = x + vx;
         var y2 = y + vy;
         while (x2 > vx && x2 < 1000 - vx && y2 > vy && y2 < 1000 - vy) {
            x2 += vx;
            y2 += vy;
         }
         var x3 = bestF.xIntersect(f2);
         var y3 = bestF.a * x3 + bestF.b;
         var t3 = bestF.timeTo(x3);

         // out.printf("type:arrow,x1:%d,y1:%d,x2:%d,y2:%d,color:red\n", x, y, x2, y2);
         String color = vy > 0 ? "red" : "blue";
         long len = (long) Math.sqrt(vx * vx + vy * vy);
         // out.printf("type:arrow,x1:%d,y1:%d,x2:%d,y2:%d,color:%s\n", x, y,
         // x+10*vx/len, y+10*vy/len, color);
         if (f2.isInFuture(x3))
            out.printf("type:arrow,x1:%d,y1:%d,x2:%d,y2:%d,color:%s\n", (int) x, (int) y, (long) (x3), (long) (y3),
                  color);

      }
      out.println("ENDSTATE");
   }

   /** Check correctness of a potential candidate for a solution */
   private static boolean check(long origX, long origY, long origZ, long vx, long vy, long vz) {
      for (int i = 0; i < points.length; ++i) {
         var f = lines[points[i].id];
         long t = (origX - f.xl) / (f.vxl - vx);
         long ox = origX + t * vx;
         long oy = origY + t * vy;
         long oz = origZ + t * vz;
         long fx = f.xl + t * f.vxl;
         long fy = f.yl + t * f.vyl;
         long fz = f.zl + t * f.vzl;
         System.out.printf("%d: t:%d, orig:%d,%d,%d, F: %d,%d,%d, delta: %d,%d,%d\n", i, t, ox, oy, oz, fx, fy, fz,
               ox - fx, oy - fy, oz - fz);
         if (ox - fx != 0 || oy - fy != 0 || oz - fz != 0) {
            return false;
         }
      }
      return true;
   }

   public static void main(String[] args) throws Exception {
      var lineList = new ArrayList<Line>();
      for (var line : Files.readAllLines(Path.of("aoc2023_24.txt"))) {
         var toks = line.split("[, @]+");
         long[] obj = new long[6];
         int i = 0;
         for (var tok : toks) {
            obj[i] = Long.parseLong(tok);
            ++i;
         }
         var f = new Line((double) obj[0] / F, (double) obj[1] / F, obj[3], obj[4]);
         f.xl = obj[0];
         f.yl = obj[1];
         f.zl = obj[2];
         f.vxl = obj[3];
         f.vyl = obj[4];
         f.vzl = obj[5];
         System.out.printf("%s -> y=%1.1f\n", f, f.a * f.x + f.b);
         lineList.add(f);
      }
      lines = lineList.toArray(new Line[0]);
      points = new P2[lineList.size()];
      for (int i = 0; i < lineList.size(); ++i) {
         points[i] = new P2();
      }
      speeds = new double[lines.length - 1];
      SlopeResult bestResult = new SlopeResult();
      SlopeResult result = new SlopeResult();
      bestResult.lineMisses = 1000000;
      bestResult.mse = 1e9;
      bestResult.y0 = 500;
      bestResult.y1 = bestResult.y0;
      // Draw lines in a box and check the results, iteratively make the window smaller to refine the search.
      // This gives an approximation how the rock should move.
      // If we get good results, we can at least deduce the vx and vy that the rock should have.
      double window = 1000;
      double N = 100;
      while (window > 0.000001) {
         double y0 = bestResult.y0 - window / 2.0;
         double y1Start = bestResult.y1 - window / 2.0;
         double inc = window / N;
         System.out.println("Window " + (y0) + " - " + (y0 + N * inc) + ", inc: " + inc);
         for (int i = 0; i < N; ++i, y0 += inc) {
            double y1 = y1Start;
            for (int j = 0; j < N; ++j, y1 += inc) {
               evalSlope(y0, y1, result);
               if (result.compareTo(bestResult) > 0) {
                  System.out.println("Best: " + result);
                  bestResult = (SlopeResult) result.clone();
               }
            }

         }
         window *= 0.4;
      }

      var bestF = new Line(0, bestResult.y0, 2000, bestResult.y1);
      System.out.println("Best function: " + bestF);
      long vx = Math.round(bestResult.avgSpeed);
      long vy = Math.round(bestF.a * bestResult.avgSpeed);
      System.out.println("vx: " + vx + "vy:" + vy);
      evalSlope(bestResult.y0, bestResult.y1, result);
      if (bestResult.lineMisses == 0) {
         for (int i = 0; i < points.length; ++i) {
            System.out.printf("%d: %1.4f, t:%1.4f", i, points[i].x, points[i].t);
            if (i > 0) {
               System.out.printf(", vx:%1.4f", speeds[i - 1]);
            }
            System.out.println(", id:" + points[i].id + ", " + lines[points[i].id]);
         }
      }
      int p1 = -1;
      double bestV = 0;
      for (int i = 0; i < points.length - 1; ++i) {
         var sumVy = Math.abs(lines[points[i].id].vy) + Math.abs(lines[points[i + 1].id].vy);
         if (sumVy > bestV) {
            bestV = sumVy;
            p1 = i;
         }
      }
      // Now we try to find exact coordinates. We take 2 intersections we found earlier (which are
      // approximations), try exact points around this approximation until we find 2 points that
      // match the vx/vy of the rock.
      var f1 = lines[points[p1].id];
      int p2 = p1 + 1;
      var f2 = lines[points[p2].id];
      long t1 = Math.round(F * points[p1].t);
      long t2 = Math.round(F * points[p2].t);
      System.out.println("Best point: " + p1 + ", t1: " + t1);
      long x1 = f1.xl + t1 * f1.vxl;
      long y1 = f1.yl + t1 * f1.vyl;
      long x2 = f2.xl + t1 * f2.vxl;
      long y2 = f2.yl + t1 * f2.vyl;
      long deltaT = Math.round(F * (points[p2].x - points[p1].x) / vx);
      System.out.printf("x1/y1:%d,%d, x2/y2:%d,%d, t1:%d, t2: %d, t2-t1:%d, deltaT: %d\n", x1, y1, x2, y2, t1, t2,
            t2 - t1, deltaT);
      long N2 = 10000;
      for (long a = t1 - N2 / 2; a <= t1 + N2 / 2; ++a) {
         x1 = f1.xl + a * f1.vxl;
         y1 = f1.yl + a * f1.vyl;
         for (long b = t2 - N2 / 2; b <= t2 + N2 / 2; ++b) {
            x2 = f2.xl + b * f2.vxl;
            y2 = f2.yl + b * f2.vyl;
            if ((x2 - x1) % vx == 0 && (y2 - y1) % vy == 0
                  && (b - a) == (x2 - x1) / vx) {
               System.out.printf("Found it? x1/y1:%d,%d, x2/y2:%d,%d, t1:%d, t2: %d, t2-t1:%d, deltaT: %d\n", x1, y1,
                     x2, y2, a, b, b - a, (x2 - x1) / vx);
               long t12 = (x2 - x1) / vx;
               long origX = x1 - a * vx;
               long origY = y1 - a * vy;
               long z1 = f1.zl + a * f1.vzl;
               long z2 = f2.zl + b * f2.vzl;
               System.out.println("z1: " + z1 + ", z2: " + z2);
               long vz = (z2 - z1) / t12;
               long origZ = z1 - a * vz;
               System.out.printf("Orig: %d,%d,%d, vz=%d, sum: %d\n", origX, origY, origZ, vz, origX + origY + origZ);
               if (check(origX, origY, origZ, vx, vy, vz)) {
                  System.out.println("Part 2: " + (origX + origY + origZ));
               }
            }
         }
      }

      //toView(bestF);
   }

}
