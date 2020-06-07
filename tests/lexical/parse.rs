use crate::lexical::num::Float;
use crate::lexical::parse::parse_float;
use core::f64;

fn check_parse_float<F: Float>(integer: &str, fraction: &str, exponent: i32, expected: F) {
    let integer = integer.as_bytes();
    let fraction = fraction.as_bytes();
    assert!(expected == parse_float(integer, fraction, exponent));
}

#[test]
fn parse_f32_test() {
    check_parse_float("", "", 0, 0.0_f32);
    check_parse_float("1", "2345", 0, 1.2345_f32);
    check_parse_float("12", "345", 0, 12.345_f32);
    check_parse_float("12345", "6789", 0, 12345.6789_f32);
    check_parse_float("1", "2345", 10, 1.2345e10_f32);
    check_parse_float("1", "2345", -38, 1.2345e-38_f32);

    // Check expected rounding, using borderline cases.
    // Round-down, halfway
    check_parse_float("16777216", "", 0, 16777216.0_f32);
    check_parse_float("16777217", "", 0, 16777216.0_f32);
    check_parse_float("16777218", "", 0, 16777218.0_f32);
    check_parse_float("33554432", "", 0, 33554432.0_f32);
    check_parse_float("33554434", "", 0, 33554432.0_f32);
    check_parse_float("33554436", "", 0, 33554436.0_f32);
    check_parse_float("17179869184", "", 0, 17179869184.0_f32);
    check_parse_float("17179870208", "", 0, 17179869184.0_f32);
    check_parse_float("17179871232", "", 0, 17179871232.0_f32);

    // Round-up, halfway
    check_parse_float("16777218", "", 0, 16777218.0_f32);
    check_parse_float("16777219", "", 0, 16777220.0_f32);
    check_parse_float("16777220", "", 0, 16777220.0_f32);

    check_parse_float("33554436", "", 0, 33554436.0_f32);
    check_parse_float("33554438", "", 0, 33554440.0_f32);
    check_parse_float("33554440", "", 0, 33554440.0_f32);

    check_parse_float("17179871232", "", 0, 17179871232.0_f32);
    check_parse_float("17179872256", "", 0, 17179873280.0_f32);
    check_parse_float("17179873280", "", 0, 17179873280.0_f32);

    // Round-up, above halfway
    check_parse_float("33554435", "", 0, 33554436.0_f32);
    check_parse_float("17179870209", "", 0, 17179871232.0_f32);

    // Check exactly halfway, round-up at halfway
    check_parse_float("1", "00000017881393432617187499", 0, 1.0000001_f32);
    check_parse_float("1", "000000178813934326171875", 0, 1.0000002_f32);
    check_parse_float("1", "00000017881393432617187501", 0, 1.0000002_f32);
}

#[test]
fn parse_f64_test() {
    check_parse_float("", "", 0, 0.0_f64);
    check_parse_float("1", "2345", 0, 1.2345_f64);
    check_parse_float("12", "345", 0, 12.345_f64);
    check_parse_float("12345", "6789", 0, 12345.6789_f64);
    check_parse_float("1", "2345", 10, 1.2345e10_f64);
    check_parse_float("1", "2345", -308, 1.2345e-308_f64);

    // Check expected rounding, using borderline cases.
    // Round-down, halfway
    check_parse_float("9007199254740992", "", 0, 9007199254740992.0_f64);
    check_parse_float("9007199254740993", "", 0, 9007199254740992.0_f64);
    check_parse_float("9007199254740994", "", 0, 9007199254740994.0_f64);

    check_parse_float("18014398509481984", "", 0, 18014398509481984.0_f64);
    check_parse_float("18014398509481986", "", 0, 18014398509481984.0_f64);
    check_parse_float("18014398509481988", "", 0, 18014398509481988.0_f64);

    check_parse_float("9223372036854775808", "", 0, 9223372036854775808.0_f64);
    check_parse_float("9223372036854776832", "", 0, 9223372036854775808.0_f64);
    check_parse_float("9223372036854777856", "", 0, 9223372036854777856.0_f64);

    check_parse_float(
        "11417981541647679048466287755595961091061972992",
        "",
        0,
        11417981541647679048466287755595961091061972992.0_f64,
    );
    check_parse_float(
        "11417981541647680316116887983825362587765178368",
        "",
        0,
        11417981541647679048466287755595961091061972992.0_f64,
    );
    check_parse_float(
        "11417981541647681583767488212054764084468383744",
        "",
        0,
        11417981541647681583767488212054764084468383744.0_f64,
    );

    // Round-up, halfway
    check_parse_float("9007199254740994", "", 0, 9007199254740994.0_f64);
    check_parse_float("9007199254740995", "", 0, 9007199254740996.0_f64);
    check_parse_float("9007199254740996", "", 0, 9007199254740996.0_f64);

    check_parse_float("18014398509481988", "", 0, 18014398509481988.0_f64);
    check_parse_float("18014398509481990", "", 0, 18014398509481992.0_f64);
    check_parse_float("18014398509481992", "", 0, 18014398509481992.0_f64);

    check_parse_float("9223372036854777856", "", 0, 9223372036854777856.0_f64);
    check_parse_float("9223372036854778880", "", 0, 9223372036854779904.0_f64);
    check_parse_float("9223372036854779904", "", 0, 9223372036854779904.0_f64);

    check_parse_float(
        "11417981541647681583767488212054764084468383744",
        "",
        0,
        11417981541647681583767488212054764084468383744.0_f64,
    );
    check_parse_float(
        "11417981541647682851418088440284165581171589120",
        "",
        0,
        11417981541647684119068688668513567077874794496.0_f64,
    );
    check_parse_float(
        "11417981541647684119068688668513567077874794496",
        "",
        0,
        11417981541647684119068688668513567077874794496.0_f64,
    );

    // Round-up, above halfway
    check_parse_float("9223372036854776833", "", 0, 9223372036854777856.0_f64);
    check_parse_float(
        "11417981541647680316116887983825362587765178369",
        "",
        0,
        11417981541647681583767488212054764084468383744.0_f64,
    );

    // Rounding error
    // Adapted from failures in strtod.
    check_parse_float("2", "2250738585072014", -308, 2.2250738585072014e-308_f64);
    check_parse_float("2", "2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187499", -308, 2.225073858507201e-308_f64);
    check_parse_float("2", "22507385850720113605740979670913197593481954635164564802342610972482222202107694551652952390813508791414915891303962110687008643869459464552765720740782062174337998814106326732925355228688137214901298112245145188984905722230728525513315575501591439747639798341180199932396254828901710708185069063066665599493827577257201576306269066333264756530000924588831643303777979186961204949739037782970490505108060994073026293712895895000358379996720725430436028407889577179615094551674824347103070260914462157228988025818254518032570701886087211312807951223342628836862232150377566662250398253433597456888442390026549819838548794829220689472168983109969836584681402285424333066033985088644580400103493397042756718644338377048603786162277173854562306587467901408672332763671875", -308, 2.2250738585072014e-308_f64);
    check_parse_float("2", "2250738585072011360574097967091319759348195463516456480234261097248222220210769455165295239081350879141491589130396211068700864386945946455276572074078206217433799881410632673292535522868813721490129811224514518898490572223072852551331557550159143974763979834118019993239625482890171070818506906306666559949382757725720157630626906633326475653000092458883164330377797918696120494973903778297049050510806099407302629371289589500035837999672072543043602840788957717961509455167482434710307026091446215722898802581825451803257070188608721131280795122334262883686223215037756666225039825343359745688844239002654981983854879482922068947216898310996983658468140228542433306603398508864458040010349339704275671864433837704860378616227717385456230658746790140867233276367187501", -308, 2.2250738585072014e-308_f64);
    check_parse_float("179769313486231580793728971405303415079934132710037826936173778980444968292764750946649017977587207096330286416692887910946555547851940402630657488671505820681908902000708383676273854845817711531764475730270069855571366959622842914819860834936475292719074168444365510704342711559699508093042880177904174497791", "9999999999999999999999999999999999999999999999999999999999999999999999", 0, 1.7976931348623157e+308_f64);
    check_parse_float("7", "4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984374999", -324, 5.0e-324_f64);
    check_parse_float("7", "4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375", -324, 1.0e-323_f64);
    check_parse_float("7", "4109846876186981626485318930233205854758970392148714663837852375101326090531312779794975454245398856969484704316857659638998506553390969459816219401617281718945106978546710679176872575177347315553307795408549809608457500958111373034747658096871009590975442271004757307809711118935784838675653998783503015228055934046593739791790738723868299395818481660169122019456499931289798411362062484498678713572180352209017023903285791732520220528974020802906854021606612375549983402671300035812486479041385743401875520901590172592547146296175134159774938718574737870961645638908718119841271673056017045493004705269590165763776884908267986972573366521765567941072508764337560846003984904972149117463085539556354188641513168478436313080237596295773983001708984375001", -324, 1.0e-323_f64);
    check_parse_float("", "0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024703282292062327208828439643411068618252990130716238221279284125033775363510437593264991818081799618989828234772285886546332835517796989819938739800539093906315035659515570226392290858392449105184435931802849936536152500319370457678249219365623669863658480757001585769269903706311928279558551332927834338409351978015531246597263579574622766465272827220056374006485499977096599470454020828166226237857393450736339007967761930577506740176324673600968951340535537458516661134223766678604162159680461914467291840300530057530849048765391711386591646239524912623653881879636239373280423891018672348497668235089863388587925628302755995657524455507255189313690836254779186948667994968324049705821028513185451396213837722826145437693412532098591327667236328125", 0, 0.0_f64);

    // Rounding error
    // Adapted from:
    //  https://www.exploringbinary.com/how-glibc-strtod-works/
    check_parse_float("", "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022250738585072008890245868760858598876504231122409594654935248025624400092282356951787758888037591552642309780950434312085877387158357291821993020294379224223559819827501242041788969571311791082261043971979604000454897391938079198936081525613113376149842043271751033627391549782731594143828136275113838604094249464942286316695429105080201815926642134996606517803095075913058719846423906068637102005108723282784678843631944515866135041223479014792369585208321597621066375401613736583044193603714778355306682834535634005074073040135602968046375918583163124224521599262546494300836851861719422417646455137135420132217031370496583210154654068035397417906022589503023501937519773030945763173210852507299305089761582519159720757232455434770912461317493580281734466552734375", 0, 2.2250738585072011e-308_f64);

    // Rounding error
    // Adapted from test-parse-random failures.
    check_parse_float("1009", "", -31, 1.009e-28_f64);
    check_parse_float("18294", "", 304, f64::INFINITY);

    // Rounding error
    // Adapted from a @dangrabcad's issue #20.
    check_parse_float("7", "689539722041643", 164, 7.689539722041643e164_f64);
    check_parse_float("768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "", 0, 7.689539722041643e164_f64);
    check_parse_float("768953972204164300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "0", 0, 7.689539722041643e164_f64);

    // Check other cases similar to @dangrabcad's issue #20.
    check_parse_float("9223372036854776833", "0", 0, 9223372036854777856.0_f64);
    check_parse_float(
        "11417981541647680316116887983825362587765178369",
        "0",
        0,
        11417981541647681583767488212054764084468383744.0_f64,
    );
    check_parse_float("9007199254740995", "0", 0, 9007199254740996.0_f64);
    check_parse_float("18014398509481990", "0", 0, 18014398509481992.0_f64);
    check_parse_float("9223372036854778880", "0", 0, 9223372036854779904.0_f64);
    check_parse_float(
        "11417981541647682851418088440284165581171589120",
        "0",
        0,
        11417981541647684119068688668513567077874794496.0_f64,
    );

    // Check other cases ostensibly identified via proptest.
    check_parse_float("71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "0", 0, 71610528364411830000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64);
    check_parse_float("126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "0", 0, 126769393745745060000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64);
    check_parse_float("38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", "0", 0, 38652960461239320000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000.0_f64);
}
