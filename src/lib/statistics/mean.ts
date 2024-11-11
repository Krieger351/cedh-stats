export const mean = <T = number>(
  data: T[],
  selector: (val: T) => number = Number,
): number => {
  let sum = 0;
  for (const value of data) {
    sum += selector(value);
  }
  return sum / data.length;
};
