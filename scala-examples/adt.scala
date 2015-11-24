object Adt {
  /** A product type */
  case class Point(
    val x: Double,
    val y: Double
  ) {
    def distanceFromOrigin: Double = {
      math.sqrt(math.pow(x, 2) + math.pow(y, 2))
    }
  }

  /** A sum type */
  sealed abstract class Option[+A] extends Product with Serializable {
  }
  case object None extends Option[Nothing] {
    def isEmpty = true
  }
  final case class Some[+A](x: A) extends Option[A] {
    def isEmpty = false
  }

}
