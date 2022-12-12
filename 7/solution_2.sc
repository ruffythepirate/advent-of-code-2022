import scala.io.StdIn

var nextLine = StdIn.readLine()

val cdRegex = """\$ cd (.+)""".r
val lsRegex = """\$ ls""".r

val folderRegex = """dir (.+)""".r
val fileRegex = """(\d+) (.+)""".r
val rootNode = new Node("/")
var currentNode = rootNode

while (nextLine != null) {
  nextLine match {
    case cdRegex(dir) => 
      if( dir == "/" ) {
        currentNode = rootNode
      } else if ( dir == ".." ) {
        currentNode = currentNode.parent
      } else {
        currentNode = currentNode.children.find(_.name == dir).get
      }
      nextLine = StdIn.readLine()
    case lsRegex() => 
      nextLine = StdIn.readLine()
      var lsResult = List[String]()
      while (nextLine != null && !nextLine.startsWith("$")) {
        lsResult = lsResult :+ nextLine
        nextLine = StdIn.readLine()
      }
      currentNode.applyLs(lsResult)
    case _ => 
      println("invalid")
      nextLine = StdIn.readLine()
  }
}

val threshold = 30000000 - (70000000- rootNode.getSize)
println(threshold)
val smallestChild = rootNode.findSmallestChildAboveThreshold(threshold)

val allChildren = rootNode.allChildren

println(allChildren.filter(_.getSize >= threshold).sortBy(_.getSize).map(node => s"${node.name} ${node.getSize}"))



class Node(val name: String, var files: List[Int], var children: List[Node], val parent: Node) {
  def this(name: String) = this(name, List(), List(), null)
  def this(name: String, parent: Node) = this(name, List(), List(), parent)

  def applyLs(output: List[String]) = {
    if(files.isEmpty && children.isEmpty) {
      val files = output.filter(fileRegex.matches(_))
      val nodes = output.filter(folderRegex.matches(_))
      val fileSizes = files.map(fileRegex.findFirstMatchIn(_).get.group(1)).map(_.toInt)
      val childNodes = nodes.map(folderRegex.findFirstMatchIn(_).get.group(1)).map(new Node(_, this))
      this.files = fileSizes
      this.children = childNodes
    }
  }

  def getSize(): Int = {
    files.sum + children.map(_.getSize()).sum
  }

  def path(): String = {
    if (parent == null) {
      name
    } else {
      parent.path() + "/" + name
    }
  }

  def findSmallestChildAboveThreshold(threshold: Int): Node = {
    val smallestChildren = children.filter(_.getSize() >= threshold).map(_.findSmallestChildAboveThreshold(threshold))
    return if (smallestChildren.isEmpty) this else smallestChildren.minBy(_.getSize())
  }

  def allChildren(): List[Node] = {
    children ++ children.flatMap(_.allChildren())
  }

  def findSmallChildren(): List[Node] = {
    if (children.isEmpty) {
      List()
    } else {
      children.filter(_.getSize() < 100000) ++ children.flatMap(_.findSmallChildren())
    }
  }
}

