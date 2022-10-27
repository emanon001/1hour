/**
 * 階乗
 * @param {number} n 
 * @return {number}
 */
const factorial = (n) => {
  if (n < 0) throw new Error('negative number');
  if (n === 0) {
    return 1;
  }
  return n * factorial(n - 1);
};

console.log(factorial(10)); // 3628800
