// FizzBuzz example
const start = performance.now();
for (let i = 1; i <= 100; i++) {
    let output = "";
    if (i % 3 === 0) {
        output += "Fizz";
    }
    if (i % 5 === 0) {
        output += "Buzz";
    }
    if (output === "") {
        output = i.toString();
    }
    console.log(output);
}

const end = performance.now();
console.log(`End: ${end}`);
console.log(`Execution time: ${end - start}ms`);
