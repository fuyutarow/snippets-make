
const gcd = (a: number, b: number) => {
  return b == 0 ? a : gcd(b, a % b)
};

const factional = (n: number) => {
  return n <= 1 ? 1 : n * factional(n - 1);
};

const ans = factional(10);
console.log(ans);



