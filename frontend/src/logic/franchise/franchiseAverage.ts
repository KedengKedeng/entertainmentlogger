export function franchiseAverage(data: any) {
  let sum = 0;
  for (let i = 0; i < data.length; i++) {
    sum += data[i].rating;
  }
  return (sum / data.length).toFixed(2);
}
