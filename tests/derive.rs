#[test]
fn derive ()
{
	let t = trybuild::TestCases::new ();

	t . pass ("tests/derive/negs.rs");
	t . pass ("tests/derive/abss.rs");
	t . pass ("tests/derive/recips.rs");
	t . pass ("tests/derive/sqrts.rs");
	t . pass ("tests/derive/exps.rs");
	t . pass ("tests/derive/lns.rs");
	t . pass ("tests/derive/sins.rs");
	t . pass ("tests/derive/coss.rs");
	t . pass ("tests/derive/tans.rs");

	t . pass ("tests/derive/sin_coss.rs");

	t . pass ("tests/derive/adds.rs");
	t . pass ("tests/derive/subs.rs");
	t . pass ("tests/derive/muls.rs");
	t . pass ("tests/derive/divs.rs");
	t . pass ("tests/derive/pows.rs");
	t . pass ("tests/derive/logs.rs");

	t . pass ("tests/derive/add_assigns.rs");
	t . pass ("tests/derive/sub_assigns.rs");
	t . pass ("tests/derive/mul_assigns.rs");
	t . pass ("tests/derive/div_assigns.rs");
	t . pass ("tests/derive/pow_assigns.rs");
	t . pass ("tests/derive/log_assigns.rs");

	t . pass ("tests/derive/scalar_adds.rs");
	t . pass ("tests/derive/scalar_subs.rs");
	t . pass ("tests/derive/scalar_muls.rs");
	t . pass ("tests/derive/scalar_divs.rs");
	t . pass ("tests/derive/scalar_pows.rs");
	t . pass ("tests/derive/scalar_logs.rs");

	t . pass ("tests/derive/scalar_add_assigns.rs");
	t . pass ("tests/derive/scalar_sub_assigns.rs");
	t . pass ("tests/derive/scalar_mul_assigns.rs");
	t . pass ("tests/derive/scalar_div_assigns.rs");
	t . pass ("tests/derive/scalar_pow_assigns.rs");
	t . pass ("tests/derive/scalar_log_assigns.rs");

	t . pass ("tests/derive/zero.rs");
	t . pass ("tests/derive/one.rs");
	t . pass ("tests/derive/e.rs");
	t . pass ("tests/derive/pi.rs");
	t . pass ("tests/derive/inf.rs");
	t . pass ("tests/derive/nan.rs");

	t . pass ("tests/derive/addition_is_commutative.rs");
	t . pass ("tests/derive/multiplication_is_commutative.rs");
}
