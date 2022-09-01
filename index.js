const isPrime = (num) => {
  if (num === 1) return false;
  if (num === 2) return true;
  for (let i = 2; i < num; i++) {
    if (num % i === 0) return false;
  }
  return true;
};

const start = new Date();
setTimeout(() => {
  console.log(isPrime(100000007));
  const end = new Date() - start;
  console.log(end / 1000 + "s");
});
