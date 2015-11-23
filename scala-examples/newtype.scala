object Newtype {
  case class Inches(self: Int) extends AnyVal {
    def +(other: Inches): Inches = {
      Inches(self + other.self)
    }
  }

  case class Feet(self: Int) extends AnyVal {
    def +(other: Feet): Feet = {
      Feet(self + other.self)
    }
  }

  def main(args: Array[String]) {
    val i = Inches(1)
    val i2 = Inches(2)
    val f = Feet(1)
    val f2 = Feet(2)
    println(s"f + f2 = ${f + f2}")
    println(s"i + i2 = ${i + i2}")
    // COMPILE error: type mismatch
    //println(s"f + i = ${f + i}")
  }
}
