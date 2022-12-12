import scala.io.StdIn

var nextLine = StdIn.readLine()

var totalScore = 0
while (nextLine != null) {
  val contentSize = nextLine.length / 2
  val firstContent = nextLine.substring(0, contentSize)
  val secondContent = nextLine.substring(contentSize)
  val matches = findMatchingItems(firstContent.toArray, secondContent.toArray)
  val sumScore = matches.map { scoreLetter(_) }.sum
  totalScore += sumScore

  nextLine = StdIn.readLine()
}
println(totalScore)

def findMatchingItems(rucksackOne: Array[Char], rucksackTwo: Array[Char]): Set[Char] = {
  var matchingItems = Set[Char]()
  for (i <- 0 until rucksackOne.length) {
    for(j <- 0 until rucksackTwo.length) {
      if (rucksackOne(i) == rucksackTwo(j)) {
        matchingItems = matchingItems + rucksackOne(i)
      }
    }
  }
  return matchingItems
}

def scoreLetter(letter: Char): Int = {
  if(letter.isUpper) {
    letter.toInt - 64 + 26
  } else {
    letter.toInt - 96 
  }
}
