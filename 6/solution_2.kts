
val input = readln()

val charRange = 14

for (i in 0 until input.length) {
  var duplicateFound = false
  for (j in 0..(charRange - 1)) {
    for (k in 0..(charRange - 1)) {
      if( k == j) continue
      if (input[i + j] == input[i + k]) {
        duplicateFound = true
        break
      }
    }
  }
  if(!duplicateFound) {
    println(i + charRange)
    break
  }
}

