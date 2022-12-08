import scala.io.StdIn
import java.util.ArrayList

var nextLine = StdIn.readLine()

var totalScore = 0
while (nextLine != null) {
  val firstBag = nextLine
  val secondBag = StdIn.readLine()
  val thirdBag = StdIn.readLine()
  val firstSet = makeSet(firstBag.toArray)
  val secondSet = makeSet(secondBag.toArray)
  val thirdSet = makeSet(thirdBag.toArray)
  val commonBadge = firstSet.intersect(secondSet).intersect(thirdSet)
  totalScore += commonBadge.map { scoreLetter(_) }.toList.sum

  nextLine = StdIn.readLine()
}
println(totalScore)

def makeSet(array: Array[Char]): Set[Char] = {
  var set = Set[Char](array: _*)
  set
}

def scoreLetter(letter: Char): Int = {
  if(letter.isUpper) {
    letter.toInt - 64 + 26
  } else {
    letter.toInt - 96 
  }
}
