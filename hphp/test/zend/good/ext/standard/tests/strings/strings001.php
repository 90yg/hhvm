<?hh
/* Do not change this test it is a README.TESTING example. */
<<__EntryPoint>> function main(): void {
$s = "alabala nica".chr(0)."turska panica";
var_dump(strstr($s, "nic"));
var_dump(strrchr($s," nic"));
}
