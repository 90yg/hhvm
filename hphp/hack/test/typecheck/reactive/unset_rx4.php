<?hh
<<file: __EnableUnstableFeatures('coeffects_provisional')>>

class A {
  public ?int $v;
}

<<__Rx>>
function f(dict<int, A> $a)[]: void {
  // OK
  unset($a[0]);
}
