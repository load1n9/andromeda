// FizzBuzz example
// deno run bench/fizzbuzz.ts
// node bench/fizzbuzz.ts
// andromeda run bench/fizzbuzz.ts

for (let i = 1; i <= 100; i++) {
    let output = "";
    if (i % 3 === 0) {
        output += "Fizz";
    }
    if (i % 5 === 0) {
        output += "Buzz";
    }
    console.log(output || i);
}
