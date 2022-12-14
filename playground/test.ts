const a : Number[] = []
// b can now modify a 
const b = a 

b.push(1) 
console.log(a) // [1]
console.log(b) // [1]