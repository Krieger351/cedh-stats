const getMedian = <T>(data: T[], extract: (datum: T) => number): number => {
  const mid = Math.floor(data.length / 2);
  if (data.length % 2 === 0) {
    return (extract(data[mid - 1]) + extract(data[mid])) / 2;
  }
  return extract(data[mid]);
};

export const getQuartiles = <T>(
  data: T[],
  extract: (datum: T) => number,
): { Q1: number; Q3: number } => {
  const mid = Math.floor(data.length / 2);
  const lowerHalf = data.slice(0, mid);
  const upperHalf = data.slice(data.length % 2 === 0 ? mid : mid + 1);

  return {
    Q1: getMedian(lowerHalf, extract),
    Q3: getMedian(upperHalf, extract),
  };
};

export const filterOutliers = <T>(
  data: T[],
  extract: (datum: T) => number,
): T[] => {
  const sortedData = data.slice().sort((a, b) => extract(a) - extract(b));
  const { Q1, Q3 } = getQuartiles(sortedData, extract);
  const IQR = Q3 - Q1;

  const lowerBound = Q1 - 1.5 * IQR;
  const upperBound = Q3 + 1.5 * IQR;

  return data.filter(
    (value) =>
      typeof extract(value) === "number" &&
      !(extract(value) < lowerBound || extract(value) > upperBound),
  );
};
