object Nullptr {
  class Bad {
    def blowup(opt: Option[Int]) {
      opt.map(x => println(s"I got an $x"))
    }

    val x = Some(23)
    // scala would complain if this was a local variable instead of a class variable
    var y: Option[Int] = _

    blowup(x)

    // RUNTIME error: java.lang.NullPointerException
    blowup(y)
  }

  def main(args: Array[String]) {
    val b = new Bad()
  }
}
