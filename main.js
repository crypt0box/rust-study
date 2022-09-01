const isPrime = (num) => {
  if (num === 1) return false;
  if (num === 2) return true;
  for (let i = 2; i < num; i++) {
    if (num % i === 0) return false;
  }
  return true;
};

const start = process.hrtime();
console.log(isPrime(100000007));
const end = process.hrtime(start);
console.log(Math.round(end[1] / 1000000) + "ms");
