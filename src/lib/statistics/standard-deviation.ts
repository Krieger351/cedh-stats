import { mean } from "@/lib/statistics/mean";

export const standardDeviation = <T>(
  data: T[],
  selector: (val: T) => number = Number,
) => {
  const mean_val = mean(data, selector);
  let acc = 0;

  for (const value of data) {
    acc = acc + Math.pow(selector(value) - mean_val, 2);
  }

  const variance = acc / data.length;

  return Math.sqrt(variance);
};
