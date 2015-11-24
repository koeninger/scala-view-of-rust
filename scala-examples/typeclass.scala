object Json {
  trait ToJson[T] {
    def toJsonStr(t: T): String
  }

  implicit object IntToJson extends ToJson[Int] {
    def toJsonStr(i: Int) = i.toString
  }

  implicit def ArrayToJson[S: ToJson] = new ToJson[Array[S]] {
    def toJsonStr(a: Array[S]) = {
      a.map(x => implicitly[ToJson[S]].toJsonStr(x)).
        mkString("[", ", ", "]")
    }
  }

  implicit class ToJsonSyntax[S: ToJson](s: S) {
    def toJsonStr = implicitly[ToJson[S]].toJsonStr(s)
  }
}

object TypeclassExample {
  import Json._
  def main(args: Array[String]) {
    val v = Array(1, 2, 3, 4)
    println(v.toJsonStr)
  }
}
