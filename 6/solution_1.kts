
val input = readln()

for (i in 0 until input.length) {
  var duplicateFound = false
  for (j in 0..3) {
    for (k in 0..3) {
      if( k == j) continue
      if (input[i + j] == input[i + k]) {
        duplicateFound = true
        break
      }
    }
  }
  if(!duplicateFound) {
    println(i + 4)
    break
  }
}

