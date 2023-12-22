ThisBuild / scalaVersion := "3.3.1"

run / fork := true
run / connectInput := true

val adventOfCode = (project in file(""))
  .settings(
    libraryDependencies += "org.typelevel" %% "cats-parse" % "1.0.0"
  )