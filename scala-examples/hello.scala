import scala.io.StdIn

object Hello {
  def main(args: Array[String]) {
    println("What's your name?")
    StdIn.readLine() match {
      case name: String => println(s"Hello, $name")
      case _ => println("Sorry, I didn't hear your null pointer")
    }
  }
}
