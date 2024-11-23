// TODO: maybe save this in the backend?
export function franchiseRatingAverage<T extends { rating: number }>(data: T[]) {
  let sum = 0;
  for (let i = 0; i < data.length; i++) {
    sum += data[i].rating;
  }
  return (sum / data.length).toFixed(2);
}