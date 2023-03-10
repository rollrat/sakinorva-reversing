# sakinorva-reversing

Sakinorva Cognitive Function Test Reversing Project

https://sakinorva.net/functions?lang=kr

## Plan

```
1. [O] Correlation between questions and mbti features
2. [O] A genetic algorithm based question selector to create specific mbti types
```

## Example

```rs
#[tokio::main]
async fn main() {
    let mbti = get_functions_from_features(Features {
        // range is -12 ~ 12
        // safe range is -11 ~ 11
        ti: 0,
        te: 0,
        si: 0,
        se: 0,
        ni: 0,
        ne: 0,
        fi: 0,
        fe: 0,
    })
    .await;

    println!("{:#?}", mbti.parse_features());
    println!("{}", mbti.parse_myers_letter_type());
}
```

```
{
    "Ne": 23.0,
    "Ni": 23.0,
    "Fi": 23.0,
    "Se": 23.0,
    "Si": 23.0,
    "Ti": 23.0,
    "Fe": 23.0,
    "Te": 23.0,
}
ENFJ
```

## Irregulation Question

```
q47, 당신은 카리스마를 내뿜고, 남들에게 매력적으로 보인다. {
    "Ne": 22.2,
    "Fi": 23.0,
    "Fe": 25.0,
    "Ti": 23.0,
    "Ni": 23.8,
    "Si": 23.0,
    "Te": 23.0,
    "Se": 23.0,
}
q69, 당신은 남들에게 예민하고 우울한 사람으로 보이기도 한다. {
    "Te": 23.0,
    "Se": 23.0,
    "Ne": 24.2,
    "Ni": 23.0,
    "Si": 23.0,
    "Ti": 23.0,
    "Fe": 23.0,
    "Fi": 25.0,
}
```

## Genetic Algorithm

### Cosine Distance + Tangent

This result is incorrect, cuz that the cosine distance only determines
the similarity of the directions of the vectors. Therefore, it is an
inappropriate method for generating the most suitable mbti.

```rs
#[tokio::main]
async fn main() {
    let mut pool = GeneticField::new(
        // ENFJ
        MbtiFitness::new(1.0, 1.0, 1.0, 1.0),
        GeneticFieldStrategy::Tangent,
        0.01,
        100,
    );

    for i in 0..100 {
        println!("Loop {}", i + 1);
        pool.evolution().await;
    }
}
```

```
Loop 1
ENTJ ( 0.63147) ESTP (-0.15809) INTJ (-0.03019) ISTJ (-0.45465) ENFJ ( 0.90920) INFJ ( 0.69629) INTP (-0.80673) ESTJ ( 0.12048) ENTP (-0.52140) INFP (-0.34421)
ENTJ ( 0.56919) ESFP (-0.05749) ENTP ( 0.64589) INTP (-0.13297) ISTJ (-0.49570) INFP ( 0.39105) ESTJ ( 0.33973) ESTJ (-0.49796) ESTJ ( 0.30702) ESFP ( 0.13821)
INTP ( 0.01230) ESFJ ( 0.60350) ESTP (-0.63379) ISTJ (-0.39929) INFJ (-0.09232) ESFJ ( 0.74718) ESTJ (-0.04380) ESFJ ( 0.64724) INTJ ( 0.20518) ISTJ (-0.27860)
INTJ (-0.65389) ISTP (-0.86792) ESFJ ( 0.23721) INTP (-0.60693) ISFJ (-0.33289) ENTJ ( 0.60418) INTJ (-0.24435) ENFJ ( 0.99102) ESFJ ( 0.59637) ENFJ ( 0.65998)
ESFJ ( 0.44500) ESFJ ( 0.32808) ESTJ ( 0.07943) ESFJ ( 0.37207) ENTP (-0.40906) ISTP (-0.82674) ESFJ ( 0.54860) ESFJ ( 0.55346) ENFP ( 0.80834) ENFJ ( 0.92129)
INFP ( 0.26256) ISTJ (-0.62153) ENFP ( 0.41633) ESFJ ( 0.59003) ENFJ ( 0.87965) ISTJ (-0.30088) ENFP ( 0.54638) ENTJ ( 0.70979) ESFJ ( 0.49108) ENFP ( 0.36991)
ISFJ (-0.29701) ESFJ ( 0.73328) ISFJ ( 0.19795) ESTP (-0.64201) ISTP (-0.83037) ESTJ ( 0.12091) ISTJ (-0.65722) ESFJ ( 0.81031) ISTJ ( 0.11679) ISTJ (-0.55541)
INTP (-0.63018) INTJ (-0.25280) ISTJ (-0.62502) ESTJ (-0.29732) ESFJ ( 0.31741) ENFJ ( 0.80190) ESTP (-0.58444) INTJ ( 0.10153) INTJ (-0.51635) INTJ (-0.32739)
ISTJ (-0.32802) INTP (-0.48088) ESFJ ( 0.15896) ENFJ ( 0.78214) ISTP (-0.81437) INTP (-0.15822) ESFP ( 0.36985) ENTJ ( 0.31461) INTJ ( 0.05810) ENFP ( 0.67312)
ENTJ ( 0.58495) INFP (-0.45841) ENTP ( 0.11168) ISTJ (-0.63457) INTP (-0.15953) ENFJ ( 0.87191) ESFJ ( 0.59969) ENFP ( 0.82878) ESTJ ( 0.10745) INTJ ( 0.60625)
Loop 2
ESFJ ( 0.41983) ENFJ ( 0.89052) ENFJ ( 0.94176) ENFJ ( 0.88299) ESTJ ( 0.03055) ESFJ ( 0.50662) ISFJ ( 0.10813) ENFJ ( 0.80190) ESFJ ( 0.70156) ENFJ ( 0.90623)
ESFJ ( 0.65080) ENFJ ( 0.91289) ENFJ ( 0.92120) INFJ ( 0.72488) ESFJ ( 0.81031) ESTJ ( 0.58845) ENFJ ( 0.92052) INFJ ( 0.37779) INFP ( 0.39903) INTJ ( 0.62048)
ENFP ( 0.67796) ENFP ( 0.71968) ENFJ ( 0.90435) ESFJ ( 0.38515) ENFP ( 0.33657) ESFJ ( 0.63809) ENFJ ( 0.91688) ENFJ ( 0.90544) ESTJ ( 0.29164) ENFP ( 0.58919)
INFJ ( 0.61167) INFJ ( 0.55421) INFJ ( 0.74689) ENFJ ( 0.97033) ENTP ( 0.22730) INFJ ( 0.62536) INFJ ( 0.77479) ESFJ ( 0.09609) ENFJ ( 0.93540) ENFJ ( 0.92823)
ESTJ ( 0.15029) ENFJ ( 0.77769) ENFP ( 0.27741) ESTJ ( 0.45204) ENFP ( 0.66779) ENFJ ( 0.94759) ENFP ( 0.66357) INFJ ( 0.35843) ENFJ ( 0.87191) INFP (-0.30742)
ENFP ( 0.76245) ENFJ ( 0.91867) INTJ ( 0.17390) ENFJ ( 0.83672) ENTJ ( 0.81946) INFJ ( 0.83840) ENFJ ( 0.89060) INFJ ( 0.62776) ESFJ ( 0.65619) ESFJ ( 0.55722)
ENFJ ( 0.92367) ENTJ ( 0.72011) ENFJ ( 0.81364) ENFJ ( 0.97122) ENFJ ( 0.88499) ESFJ ( 0.82125) ENTJ ( 0.53008) ENTJ ( 0.03141) ESFJ ( 0.77424) ENFJ ( 0.84500)
ESFJ ( 0.70864) ESFJ ( 0.65559) INTJ (-0.11581) ENFJ ( 0.90184) ESFJ ( 0.74446) ISFJ (-0.12678) ESFJ ( 0.66151) ENTJ ( 0.34123) ENFJ ( 0.87891) ENFP ( 0.49060)
ENFP ( 0.63870) INFJ ( 0.75422) ESTJ (-0.26883) INFJ ( 0.72080) ENFJ ( 0.78075) ENFJ ( 0.92586) ESFP ( 0.10664) ENFJ ( 0.87674) ISFJ ( 0.29483) ENFJ ( 0.99102)
ENTJ ( 0.14026) ENFJ ( 0.84755) ESFJ ( 0.76749) ENFJ ( 0.75877) INFJ ( 0.53823) ENFJ ( 0.71428) ENFJ ( 0.88636) INFJ ( 0.58837) ENFJ ( 0.97374) ENFJ ( 0.79564)
Loop 3
ENFJ ( 0.74584) ENFJ ( 0.88387) INFJ ( 0.62743) INFJ ( 0.61167) INFJ ( 0.68545) ESFJ ( 0.76226) INTJ ( 0.28509) ESFJ ( 0.68153) ENTJ ( 0.80080) ENFP ( 0.79518)
ENFJ ( 0.84413) ENFJ ( 0.89103) ENFJ ( 0.83604) INFJ ( 0.69830) ENFJ ( 0.97773) ENFJ ( 0.90949) ENFJ ( 0.84982) ENFJ ( 0.80373) ENFJ ( 0.86898) ENFJ ( 0.95711)
ENFJ ( 0.76949) ENFJ ( 0.82626) ENFJ ( 0.95821) ENFJ ( 0.79490) INFJ ( 0.79412) INTJ ( 0.01303) INFJ ( 0.24895) INFJ ( 0.66875) ENFJ ( 0.77021) ENTJ ( 0.13998)
ENFJ ( 0.91994) ENFJ ( 0.77547) ENFJ ( 0.91867) ENFJ ( 0.79212) ESFJ ( 0.76465) ESFJ ( 0.66351) ENFP ( 0.61321) ENFJ ( 0.91484) ENFJ ( 0.87889) ENFJ ( 0.97033)
ENFJ ( 0.89740) ENFJ ( 0.89352) INFJ ( 0.73202) ESTJ ( 0.37874) INFJ ( 0.65336) ENFJ ( 0.96596) INFJ (-0.01714) ENFJ ( 0.83626) INFJ ( 0.77264) ENFJ ( 0.92408)
ENTP ( 0.27861) ESFJ ( 0.42579) ENFJ ( 0.87793) ENFJ ( 0.84512) ENFJ ( 0.86721) ENFJ ( 0.92816) ENFJ ( 0.88117) ENFJ ( 0.81077) ESFJ ( 0.85276) ESFJ ( 0.77579)
ENFJ ( 0.90814) INFJ ( 0.79196) ENFJ ( 0.82607) ENFP ( 0.66647) ENFJ ( 0.93091) ENFJ ( 0.92745) ISFJ ( 0.47141) ENFJ ( 0.91178) ENFJ ( 0.86747) ENFJ ( 0.88586)
ENFJ ( 0.85096) ENFJ ( 0.93097) ENFJ ( 0.88800) INFJ ( 0.77018) ENFJ ( 0.85410) ENFJ ( 0.87850) ENFJ ( 0.82772) ENFJ ( 0.88049) ENFJ ( 0.93729) ENFJ ( 0.95863)
ENFJ ( 0.96286) ENTJ (-0.12980) ENFJ ( 0.83275) ENFP ( 0.66357) ESFJ ( 0.60861) ENFJ ( 0.79090) ENFJ ( 0.91319) ENFJ ( 0.85705) ENFJ ( 0.82047) ENFJ ( 0.79652)
ENFJ ( 0.82004) ESFJ ( 0.65217) ENFJ ( 0.93348) ENFJ ( 0.85129) ENFJ ( 0.88998) ENFJ ( 0.95610) ENFJ ( 0.73151) ENFJ ( 0.88172) ENFP ( 0.83895) INFP ( 0.27308)
Loop 4
ENFJ ( 0.90386) ENFJ ( 0.86805) ENFJ ( 0.81077) ENFJ ( 0.86280) ENFJ ( 0.95786) ENFJ ( 0.94556) ENFJ ( 0.87084) ENFJ ( 0.89942) ENFJ ( 0.91523) ENFJ ( 0.87309)
ENFJ ( 0.92909) ENFJ ( 0.86944) INTJ ( 0.28874) ENFJ ( 0.97223) ENFJ ( 0.85067) ENFJ ( 0.93643) ENFJ ( 0.72369) ESFJ ( 0.51760) ENFJ ( 0.84858) ENFJ ( 0.91471)
ENTJ ( 0.74892) ENFJ ( 0.95643) ENFP ( 0.65150) ESFJ ( 0.79301) ENFJ ( 0.98538) ENTJ ( 0.53872) ENFJ ( 0.92737) ENFJ ( 0.88969) ENFJ ( 0.92129) ENFJ ( 0.86743)
ENFJ ( 0.84432) INFJ ( 0.66873) ENFJ ( 0.94289) ENFJ ( 0.97451) ENFJ ( 0.94924) ENFJ ( 0.94488) ENFJ ( 0.84124) ENFJ ( 0.82417) ENFJ ( 0.91350) ENFJ ( 0.95711)
ENFJ ( 0.91516) ENFJ ( 0.92604) ENFJ ( 0.91932) ENFJ ( 0.91773) ENFJ ( 0.76828) ENFJ ( 0.94576) ENFJ ( 0.99729) ENFP ( 0.61925) ENFJ ( 0.83702) ENFP ( 0.84102)
ENFJ ( 0.90863) ENTJ ( 0.79828) ENFJ ( 0.85461) ENFJ ( 0.82761) ENFJ ( 0.66769) ENFJ ( 0.88259) INFJ ( 0.68657) ENFJ ( 0.90218) ENFJ ( 0.92986) ENFJ ( 0.95685)
ENFJ ( 0.86744) ENFP ( 0.72145) INFJ ( 0.71556) ENFJ ( 0.89164) INTJ (-0.03773) ESFJ ( 0.67217) ENFJ ( 0.86659) ENFJ ( 0.86982) INFJ ( 0.69537) ENFJ ( 0.96684)
ENFJ ( 0.91790) INFJ ( 0.76128) ENFJ ( 0.95416) ENFJ ( 0.81971) ENFJ ( 0.88499) ENFJ ( 0.84290) ENFJ ( 0.90814) ENFJ ( 0.97677) ENFJ ( 0.84739) ENFJ ( 0.87107)
ENFJ ( 0.96583) ENTJ ( 0.50538) ENFJ ( 0.87586) ENFJ ( 0.76267) ENFP ( 0.58236) ENFJ ( 0.95799) ESFJ ( 0.80659) ENFJ ( 0.87630) ENFJ ( 0.90831) ENFJ ( 0.87864)
ENFJ ( 0.96409) ENFJ ( 0.76852) ENFJ ( 0.94732) ENFJ ( 0.84954) ENFJ ( 0.85710) ENFJ ( 0.94633) ENFJ ( 0.87683) ENFJ ( 0.95779) ENFJ ( 0.99104) ENFJ ( 0.85284)
Loop 5
ENFJ ( 0.75287) ESFJ ( 0.79825) ENFJ ( 0.85897) ENFJ ( 0.84514) ENFJ ( 0.97340) ENFJ ( 0.79707) ENFJ ( 0.92486) ENFJ ( 0.80922) ENFJ ( 0.94963) ENFJ ( 0.85368)
ENFJ ( 0.91471) ENFJ ( 0.89269) ESFJ ( 0.73079) ENFJ ( 0.96702) ENFJ ( 0.76095) ENFJ ( 0.70993) ENFJ ( 0.93523) ENFJ ( 0.90719) ENFP ( 0.54090) ENFJ ( 0.86195)
ENFJ ( 0.77813) ENFJ ( 0.93655) ENFJ ( 0.82086) ENFJ ( 0.89692) ENFJ ( 0.78625) ENFJ ( 0.84915) ENFJ ( 0.91972) ENFJ ( 0.94045) ENFJ ( 0.80336) ENFJ ( 0.82622)
ENFJ ( 0.84668) ENFJ ( 0.94325) ENFJ ( 0.89327) ENFJ ( 0.86548) ENFJ ( 0.96417) ESFJ ( 0.64768) ENTJ ( 0.77851) ENFJ ( 0.86097) ENFJ ( 0.85583) ENFJ ( 0.89295)
ENFJ ( 0.75707) ENFJ ( 0.99325) ESFJ ( 0.67038) ENFJ ( 0.95359) ENFJ ( 0.93060) ENFJ ( 0.89242) ENFJ ( 0.95224) ENFJ ( 0.83975) ENFJ ( 0.94317) ENFJ ( 0.97686)
ENFJ ( 0.92585) ENFJ ( 0.88733) ENFJ ( 0.87514) ENFJ ( 0.89942) ENFJ ( 0.66627) ENFJ ( 0.85753) ENFJ ( 0.83518) ENFJ ( 0.88667) ENFJ ( 0.96495) ENFJ ( 0.81875)
ENFJ ( 0.86293) INFJ ( 0.80231) ENFJ ( 0.82384) ENFJ ( 0.94959) ENFJ ( 0.92086) INTJ ( 0.07639) ENFJ ( 0.81832) INFJ ( 0.65490) ENFJ ( 0.91859) INFJ ( 0.67655)
ENFJ ( 0.95996) ENFJ ( 0.87683) ENFJ ( 0.81347) ESFJ ( 0.82153) ENFJ ( 0.95748) ENFJ ( 0.79147) ENFJ ( 0.88157) INFJ ( 0.70557) ENFJ ( 0.94623) ENFJ ( 0.90412)
ENFJ ( 0.90316) ENFJ ( 0.94289) ENFJ ( 0.94860) ENFJ ( 0.91693) ENFJ ( 0.82677) ESFJ ( 0.77444) ENFJ ( 0.86955) ENFJ ( 0.94707) ENFJ ( 0.90887) ENFJ ( 0.93860)
ENFJ ( 0.91619) ENFJ ( 0.84856) ESTJ ( 0.05609) ENFJ ( 0.89942) ENFJ ( 0.96757) ENFP ( 0.74160) INFP ( 0.16245) ENFJ ( 0.84068) ENFJ ( 0.80432) ENFJ ( 0.92597)
```

### Euclidean Distance + Tangent

```rs
#[tokio::main]
async fn main() {
    let mut pool = GeneticField::new(
        // ENFJ
        MbtiFitness::new(1.0, 1.0, 1.0, 1.0),
        GeneticFieldStrategy::Tangent,
        GeneticCompareStrategy::Euclidean,
        0.01,
        100,
    );

    for i in 0..100 {
        println!("Loop {}", i + 1);
        pool.evolution().await;
    }
}
```

```
Loop 1
ISTJ ( 0.69836) INFJ ( 0.82374) INFJ ( 0.76944) ISTJ ( 0.65448) ISTP ( 0.64053) ISTP ( 0.70931) ISFP ( 0.73282) ISTJ ( 0.67202) ISTP ( 0.68277) ENFJ ( 0.80312)
INFJ ( 0.77052) ENTP ( 0.75407) ESTJ ( 0.69884) ENFJ ( 0.83196) ENFJ ( 0.79932) ENFJ ( 0.84601) ESTJ ( 0.72977) INTP ( 0.66683) INTP ( 0.67169) ISFP ( 0.73176)
ISFJ ( 0.77921) ENFJ ( 0.83921) ENTP ( 0.73026) INTJ ( 0.71498) ISTJ ( 0.71001) ESFJ ( 0.79557) ENFJ ( 0.85724) ENFP ( 0.80210) INTP ( 0.63701) ENTP ( 0.74083)
INTP ( 0.66855) ENFJ ( 0.80682) ENFJ ( 0.80612) INFJ ( 0.77131) ISTJ ( 0.66277) INTP ( 0.64449) ESFJ ( 0.76500) ESTP ( 0.63598) INFP ( 0.71458) ESFJ ( 0.77993)
ESFJ ( 0.79543) ENFP ( 0.76817) ESTJ ( 0.72317) ISFJ ( 0.78357) ESTJ ( 0.67128) INFP ( 0.73295) ENFJ ( 0.86263) ESFJ ( 0.70736) ISFP ( 0.67864) ISTJ ( 0.68905)
ISTJ ( 0.66211) ISTJ ( 0.69408) ENFP ( 0.81525) ENFP ( 0.77498) ESFP ( 0.79363) ISFP ( 0.69744) ISFJ ( 0.63481) ENFJ ( 0.82716) ENFJ ( 0.80235) ISTJ ( 0.62748)
ESTP ( 0.70370) ISTJ ( 0.72830) ENFJ ( 0.82040) ESTJ ( 0.71879) ISTP ( 0.67737) ENFP ( 0.79317) ESFJ ( 0.78799) ESTJ ( 0.77463) ESTJ ( 0.69613) INFP ( 0.74589)
ENFJ ( 0.89216) ENTJ ( 0.76246) ESFJ ( 0.76903) ESFP ( 0.74168) INFJ ( 0.76551) ENFP ( 0.77618) ISFP ( 0.69139) INFP ( 0.73657) ESTJ ( 0.72226) INTP ( 0.71819)
INFJ ( 0.82490) ESFP ( 0.74329) ISTJ ( 0.63185) ESFJ ( 0.77010) INTJ ( 0.70115) ISFJ ( 0.70102) ESTP ( 0.59913) ISFJ ( 0.68113) ESFJ ( 0.76376) ISTJ ( 0.62759)
ESFJ ( 0.81200) ESTP ( 0.70383) ISFP ( 0.71017) ESTP ( 0.65961) ENTJ ( 0.76715) ESTJ ( 0.66728) ISFJ ( 0.74695) ISTP ( 0.67879) ISTJ ( 0.58581) ESFP ( 0.74999)
Loop 2
ESFP ( 0.74168) ENFJ ( 0.86079) ESTJ ( 0.67849) ESTJ ( 0.72223) ESTJ ( 0.68446) ENFP ( 0.79285) ISFP ( 0.69515) ESTP ( 0.73375) INFP ( 0.72561) INTJ ( 0.71391)
ESTJ ( 0.74836) ISFJ ( 0.75817) ESTJ ( 0.68837) ESFP ( 0.71719) ENFJ ( 0.83726) ENFJ ( 0.82321) ENFJ ( 0.79925) INFP ( 0.73374) ISTJ ( 0.72571) INFP ( 0.75430)
ISFP ( 0.75640) ESTP ( 0.69974) INTJ ( 0.77464) ESTJ ( 0.77867) ISFP ( 0.68341) ENFJ ( 0.86868) ENTJ ( 0.80465) ENFP ( 0.79498) INFJ ( 0.76356) ENTJ ( 0.74755)
ENFJ ( 0.85875) INFP ( 0.72190) ESTP ( 0.64575) ENFJ ( 0.85873) ESTP ( 0.65300) ESFJ ( 0.76376) ISFJ ( 0.67118) ESTJ ( 0.73435) ESFJ ( 0.80101) ESFJ ( 0.79383)
ENFJ ( 0.92674) INTJ ( 0.76086) ENTJ ( 0.76694) ENFJ ( 0.84352) INFJ ( 0.80007) ISTJ ( 0.67853) ESTP ( 0.65518) ENFP ( 0.78942) INTJ ( 0.75871) ENFJ ( 0.82998)
ENFJ ( 0.86092) ENTJ ( 0.74374) ESTP ( 0.64316) ESFP ( 0.75513) ENFJ ( 0.82657) ESFJ ( 0.79081) ESFP ( 0.75900) ENFJ ( 0.85996) INFJ ( 0.78777) INFJ ( 0.76544)
ESTJ ( 0.74067) ESFJ ( 0.82784) ISTP ( 0.58826) ENFJ ( 0.82309) ENFJ ( 0.87260) INTJ ( 0.75398) ISTP ( 0.67021) ENTJ ( 0.79925) INFJ ( 0.75369) ENFJ ( 0.78407)
ESTJ ( 0.75966) ENFP ( 0.73329) INTJ ( 0.72120) ENFJ ( 0.80658) INFP ( 0.76185) ENFP ( 0.80087) ESFJ ( 0.77136) ESTJ ( 0.70971) ESTP ( 0.72618) ESFJ ( 0.80682)
ENFJ ( 0.86285) INFJ ( 0.75285) ENTP ( 0.78804) ENFP ( 0.78070) ENFJ ( 0.81558) ESTJ ( 0.66372) ESFJ ( 0.77784) ESFJ ( 0.85402) INTJ ( 0.74806) ESFJ ( 0.80325)
ESFP ( 0.76633) ENFJ ( 0.86875) ESFJ ( 0.78426) ESFJ ( 0.77789) ENTJ ( 0.78039) ESTJ ( 0.75316) ENFP ( 0.80989) ENFJ ( 0.86100) INFJ ( 0.77907) ISTJ ( 0.67420)
Loop 3
ESFJ ( 0.75865) ISTJ ( 0.67026) ESFJ ( 0.73977) ENFJ ( 0.85097) INFJ ( 0.78133) INTJ ( 0.72940) ENTJ ( 0.81930) ESFJ ( 0.78822) ESTJ ( 0.72060) INTJ ( 0.73098)
ENTP ( 0.75408) ISFP ( 0.71338) ENTJ ( 0.77899) ESFJ ( 0.81045) ENTJ ( 0.76839) ENTJ ( 0.83298) ESFJ ( 0.83455) ENTJ ( 0.81574) ENFP ( 0.81752) ENFJ ( 0.83971)
ENFJ ( 0.82763) ENFP ( 0.80865) ENFJ ( 0.82105) ENFJ ( 0.81875) ESFJ ( 0.78109) ESFP ( 0.75346) INTP ( 0.69056) ENFJ ( 0.82165) ESFJ ( 0.78515) ESFJ ( 0.79045)
ESFJ ( 0.81685) ENFJ ( 0.81885) ESFJ ( 0.82121) ENFJ ( 0.84730) INFJ ( 0.82287) ENTJ ( 0.75363) ESFJ ( 0.77784) ESFJ ( 0.80523) ISFJ ( 0.77725) ENFJ ( 0.84184)
ENFJ ( 0.81936) ESFJ ( 0.79624) ENTJ ( 0.80454) ENTJ ( 0.74621) ESFJ ( 0.84586) ESTJ ( 0.76139) INFJ ( 0.70699) ISTJ ( 0.66313) ENFJ ( 0.82074) ENFJ ( 0.97980)
ESFJ ( 0.76693) ENTJ ( 0.78814) ISTP ( 0.65795) ENFJ ( 0.86392) ENFJ ( 0.82960) ISFJ ( 0.72233) INTJ ( 0.73535) INTJ ( 0.68808) ESFP ( 0.76525) ESFJ ( 0.73797)
ENFJ ( 0.84316) ESFJ ( 0.83084) ESTJ ( 0.74892) ISTJ ( 0.70680) ESFJ ( 0.82433) ENFJ ( 0.84884) INTJ ( 0.74007) ENFJ ( 0.84847) INFJ ( 0.80318) ESFP ( 0.76608)
ESFP ( 0.71719) ENTJ ( 0.82022) ENFJ ( 0.79481) ESFJ ( 0.76721) ENFJ ( 0.79932) ENTJ ( 0.80858) ISTP ( 0.64506) ESFJ ( 0.79822) ISTJ ( 0.67098) ENFJ ( 0.83571)
ENFJ ( 0.89841) INFJ ( 0.80987) ENTJ ( 0.78520) INFJ ( 0.77153) ENFP ( 0.78414) ISTJ ( 0.70258) INTJ ( 0.67965) ESTJ ( 0.74184) ENFJ ( 0.84348) ENFJ ( 0.84316)
ENFJ ( 0.86353) ENFJ ( 0.83235) ISTJ ( 0.67600) ESTJ ( 0.73605) INFJ ( 0.73274) ENFP ( 0.80090) ESFJ ( 0.79089) ESFP ( 0.78312) ESTP ( 0.64782) ESFJ ( 0.79251)
Loop 4
ESFP ( 0.72208) ENFJ ( 0.85540) ENFP ( 0.79625) ENFJ ( 0.83718) INTJ ( 0.66997) ENFJ ( 0.86251) ENFJ ( 0.86511) INFJ ( 0.81669) ENFJ ( 0.83900) ENFJ ( 0.82493)
ENFJ ( 0.85896) ESFJ ( 0.75414) ESTJ ( 0.72879) ISTJ ( 0.71084) ENFJ ( 0.84716) ENFJ ( 0.83551) ENFJ ( 0.86768) ENFJ ( 0.86376) ESFJ ( 0.72813) ENFJ ( 0.93023)
ENFJ ( 0.84495) ENFJ ( 0.87462) ENFJ ( 0.79604) ISFJ ( 0.72998) ENFJ ( 0.85170) INFP ( 0.75295) ENFJ ( 0.83659) ENFJ ( 0.78885) INFJ ( 0.79202) ESFJ ( 0.79341)
ESFJ ( 0.82480) ENTP ( 0.76476) ESFJ ( 0.84452) ENFJ ( 0.97935) ENFJ ( 0.90819) ESFJ ( 0.79501) INTP ( 0.68784) INTP ( 0.67947) ENFJ ( 0.87119) ESFJ ( 0.78884)
ESTP ( 0.67306) ENFJ ( 0.97931) ENFJ ( 0.89838) ESFP ( 0.76204) ISTJ ( 0.73030) ENFJ ( 0.83531) ENFJ ( 0.87649) ENFP ( 0.79479) INTJ ( 0.75174) INFJ ( 0.75902)
ESFJ ( 0.86006) ISTJ ( 0.67872) ENFJ ( 0.87595) ENFJ ( 0.90233) ENTJ ( 0.78114) ENFJ ( 0.92092) ISFJ ( 0.76054) ESFJ ( 0.74050) ENFJ ( 0.85638) ISTJ ( 0.70258)
ISTJ ( 0.65760) ENFJ ( 0.94291) ENTJ ( 0.78026) ENFJ ( 0.98370) INFJ ( 0.82940) ESTP ( 0.69557) ENTJ ( 0.78588) ENFJ ( 0.86899) ENFJ ( 0.89875) ENFJ ( 0.84447)
ENFJ ( 0.96609) ESFJ ( 0.80570) ESTJ ( 0.75488) ESFJ ( 0.78163) ESFP ( 0.74832) ESFJ ( 0.84201) ENTJ ( 0.85212) INFJ ( 0.81145) ENTJ ( 0.78773) ESTJ ( 0.76836)
ISTJ ( 0.72791) INFJ ( 0.82697) ENTJ ( 0.82833) ENFJ ( 0.85984) ENFJ ( 0.95051) INFJ ( 0.79332) ENFJ ( 0.87591) ENFJ ( 0.83943) ESFJ ( 0.80014) ENTJ ( 0.76429)
ESTJ ( 0.78119) ESFJ ( 0.78388) ENFP ( 0.78968) ENFJ ( 0.84697) ISTJ ( 0.73793) ISFJ ( 0.77725) ESFJ ( 0.77007) INTJ ( 0.75235) ENFJ ( 0.89127) ENFJ ( 0.95648)
Loop 5
ENFJ ( 0.98041) ENFJ ( 0.90445) ESFJ ( 0.84730) ENFJ ( 0.97935) ENFJ ( 0.93353) ENFJ ( 0.93726) ENFP ( 0.80090) ENFJ ( 0.86500) ENFJ ( 0.90437) INTJ ( 0.72773)
ENFJ ( 0.93269) ESTP ( 0.66035) ENFJ ( 0.89264) ENFJ ( 0.92995) ENFJ ( 0.89595) ENFJ ( 0.87595) INFJ ( 0.77977) ENFJ ( 0.91508) ENFJ ( 0.87491) ISTJ ( 0.72280)
ENFJ ( 0.86868) ENFJ ( 0.84203) ESFP ( 0.77995) ENFJ ( 0.94549) ENFP ( 0.79479) ENFJ ( 0.92664) ENFJ ( 0.86402) ENFJ ( 0.82493) ESFJ ( 0.78548) ESFJ ( 0.83891)
ENFJ ( 0.90429) ENFJ ( 0.97931) ENFP ( 0.84480) INFJ ( 0.82952) ENFJ ( 0.85516) ENTP ( 0.72294) ESFJ ( 0.81037) ENFJ ( 0.87782) ENFJ ( 0.90090) ESFJ ( 0.83568)
ENFJ ( 0.86050) ENFJ ( 0.79084) ENFJ ( 0.87363) ENFJ ( 0.93886) ENFJ ( 0.91306) ENFJ ( 0.92567) ESFJ ( 0.80829) ENFJ ( 0.86827) ENFJ ( 0.87162) ENFJ ( 0.85283)
ESTJ ( 0.77107) ENFJ ( 0.86841) ENFJ ( 0.85561) ENFJ ( 0.93023) ENFJ ( 0.83718) ENFJ ( 0.94965) ENFJ ( 0.89911) ENFJ ( 0.85137) ISFJ ( 0.74645) ENTJ ( 0.81463)
ENTJ ( 0.80873) ENFJ ( 0.86658) ENFJ ( 0.81932) ENTJ ( 0.82285) ENFJ ( 0.88963) ENFJ ( 0.98370) ENFJ ( 0.84552) ENFJ ( 0.98213) ENFJ ( 0.87823) ENFJ ( 0.97586)
ENFJ ( 0.87954) ENFJ ( 0.95516) ENFJ ( 0.91334) ENFJ ( 0.88536) ENFJ ( 0.86294) ENFJ ( 0.92971) ENFJ ( 0.92934) ENFJ ( 0.95051) ENFJ ( 0.82250) ENFJ ( 0.85333)
ENFJ ( 0.98272) ENFJ ( 0.94560) ENFJ ( 0.96154) ENFJ ( 0.86278) ENFJ ( 0.88728) ENFJ ( 0.95412) ENFJ ( 0.84717) ENTJ ( 0.81253) ISFP ( 0.72170) ISTJ ( 0.67502)
ENFJ ( 0.85592) ENFJ ( 0.92880) ENFJ ( 0.89435) ENFP ( 0.76748) ENFJ ( 0.82081) ENFJ ( 0.92332) ESFJ ( 0.78883) ENFJ ( 0.91672) ENFP ( 0.82215) ENFJ ( 0.88172)
Loop 6
ENFJ ( 0.95799) ENFJ ( 0.90376) ENFJ ( 0.95253) ENFJ ( 0.86095) ENFJ ( 0.87064) ENFJ ( 0.96151) ENFJ ( 0.93353) ENFJ ( 0.98620) ENFJ ( 0.92717) ENFJ ( 0.93174)
ENFJ ( 0.84208) ESFJ ( 0.85185) ENFP ( 0.82638) ESFJ ( 0.82524) ENFJ ( 0.97931) ENFJ ( 0.92664) ENFJ ( 0.97694) ENFJ ( 0.98370) ENFP ( 0.79263) ENFJ ( 0.98630)
ENFJ ( 0.86243) ENFJ ( 0.89081) ENFJ ( 0.98375) ENFJ ( 0.98241) ENFJ ( 0.99308) ENFJ ( 0.86542) ENFJ ( 0.87533) ENFJ ( 0.91165) ENFJ ( 0.85443) INTJ ( 0.72384)
ESFJ ( 0.73040) ENFJ ( 0.90577) ENFJ ( 0.98283) ENFJ ( 0.95516) ENFJ ( 0.81748) ENFJ ( 0.96991) ENTP ( 0.72851) ENFJ ( 0.89471) ENFJ ( 0.80947) ENFJ ( 0.94270)
ENFJ ( 0.82983) ENFJ ( 0.94017) ENFJ ( 0.92340) ENFJ ( 0.90994) ENFJ ( 0.97749) ENFJ ( 0.96385) ENFJ ( 0.84261) ESFJ ( 0.83262) ENFJ ( 0.89723) ENFJ ( 0.95516)
ENFJ ( 0.94646) ENFJ ( 0.97368) ESFJ ( 0.82035) ENFJ ( 0.98212) ENFJ ( 0.92548) ENFJ ( 0.95840) ENTJ ( 0.80938) ENFJ ( 0.81052) ENFJ ( 0.98865) ENFJ ( 0.97443)
ENFJ ( 0.94717) ENFJ ( 0.91025) ENFJ ( 0.87700) ENFJ ( 0.99430) ENFJ ( 0.93528) ENFJ ( 0.88601) ENFJ ( 0.90709) ENFJ ( 0.93241) ENFJ ( 0.90303) ENFJ ( 0.86573)
ENFJ ( 0.97407) ENFJ ( 0.84099) ENFJ ( 0.92347) ENFJ ( 0.86197) ENFJ ( 0.89914) ENFJ ( 0.91702) ENFJ ( 0.94623) ENFJ ( 0.98370) ENFJ ( 0.98109) INFJ ( 0.81585)
ENFJ ( 0.98939) ENFJ ( 0.88312) ENFJ ( 0.87720) ENFJ ( 0.97944) ENFJ ( 0.97935) ENFJ ( 0.84317) ENFJ ( 0.98213) ENFJ ( 0.97891) ENFJ ( 0.91881) ENFJ ( 0.86703)
ENFJ ( 0.96545) ENFJ ( 0.93782) ENFJ ( 0.90293) ENFJ ( 0.91024) ENFP ( 0.80185) ENFJ ( 0.89941) ESFJ ( 0.81373) ENFJ ( 0.94061) ENFJ ( 0.93127) ENFJ ( 0.94371)
Loop 7
ENFJ ( 0.97313) ENFJ ( 0.94247) ENFJ ( 0.87233) ENFJ ( 0.99191) ENFJ ( 0.98630) ENFJ ( 0.99414) ENFJ ( 0.98381) ENFJ ( 0.98241) ENFJ ( 0.92760) ENFJ ( 0.98788)
ENFJ ( 0.95725) ENFJ ( 0.90420) ENFJ ( 0.98952) ENFJ ( 0.97591) ENFJ ( 0.98241) ENFJ ( 0.98261) ENFJ ( 0.92362) ENFJ ( 0.95129) ENFJ ( 0.97794) ENFJ ( 0.97912)
ENFJ ( 0.97819) ENFJ ( 0.96419) ENFJ ( 0.93331) ENFJ ( 0.79769) ENFJ ( 0.88096) ENFJ ( 0.90246) ENFJ ( 0.92367) ENFJ ( 0.97405) ENFJ ( 0.91845) ENFJ ( 0.88994)
ENFJ ( 0.92670) ENFJ ( 0.97443) ENFJ ( 0.98865) ENFJ ( 0.98939) ENFJ ( 0.83369) ENFJ ( 0.94013) ENFJ ( 0.95516) ENFJ ( 0.92778) ENFJ ( 0.98010) ENFJ ( 0.97980)
ENFJ ( 0.95516) ENFJ ( 0.97749) ENFJ ( 0.88854) ENFJ ( 0.91796) ENFJ ( 0.97496) ENFJ ( 0.93413) ENFJ ( 0.97051) ENFJ ( 0.99140) ENFJ ( 0.91675) ENFJ ( 0.97780)
ENFJ ( 0.99186) ENFJ ( 0.98545) ENFJ ( 0.96099) ENFJ ( 0.91008) ENFJ ( 0.98491) ENFJ ( 0.98302) ENFJ ( 0.98109) ENFJ ( 0.98213) ENFJ ( 0.98939) ENFJ ( 0.98213)
ENFJ ( 0.92965) ENFJ ( 0.96225) ENFJ ( 0.98630) ENFJ ( 0.98381) ENFJ ( 0.93726) ENFJ ( 0.93094) ENFJ ( 0.99287) ENFJ ( 0.96271) ESFJ ( 0.82035) ENFJ ( 0.91500)
ENFJ ( 0.98646) ENFJ ( 0.88904) ENFJ ( 0.97995) ENFJ ( 0.94197) ENFJ ( 0.91349) ENFJ ( 0.89349) ENFJ ( 0.95207) ENFJ ( 0.98370) ENFJ ( 0.88835) ENFJ ( 0.98997)
ENFJ ( 0.94741) ENFJ ( 0.96151) ENFJ ( 0.97772) ENFJ ( 0.94494) ENFJ ( 0.97382) ENFJ ( 0.98331) ENFJ ( 0.83277) ENFJ ( 0.90530) ESFJ ( 0.83262) ENFJ ( 0.87052)
ENFJ ( 0.98865) ENFJ ( 0.98584) ENFJ ( 0.94702) ENFJ ( 0.98408) ENFJ ( 0.98283) ENFJ ( 0.96991) ENFJ ( 0.97891) ENFJ ( 0.98091) ENFJ ( 0.98415) ENFJ ( 0.97425)
Loop 8
ENFJ ( 0.99381) ENFJ ( 0.99368) ENFJ ( 0.97653) ENFJ ( 0.99414) ENFJ ( 0.98054) ENFJ ( 0.98092) ENFJ ( 0.97496) ENFJ ( 0.96913) ENFJ ( 0.99414) ENFJ ( 0.98457)
ENFJ ( 0.98213) ENFJ ( 0.96099) ENFJ ( 0.87140) ENFJ ( 0.95308) ENFJ ( 0.97684) ENFJ ( 0.92313) ENFJ ( 0.95362) ENFJ ( 0.98630) ENFJ ( 0.98074) ENFJ ( 0.95762)
ENFJ ( 0.98997) ENFJ ( 0.98491) ENFJ ( 0.99414) ENFJ ( 0.96656) ENFJ ( 0.98491) ENFJ ( 0.99385) ENFJ ( 0.99984) ENFJ ( 0.95346) ENFJ ( 0.98362) ENFJ ( 0.98109)
ENFJ ( 0.98939) ENFJ ( 0.98060) ENFJ ( 0.98707) ENFJ ( 0.98907) ENFJ ( 0.98720) ENFJ ( 0.98858) ENFJ ( 0.93355) ENFJ ( 0.95516) ENFJ ( 0.97716) ENFJ ( 0.89310)
ENFJ ( 0.99286) ENFJ ( 0.99670) ENFJ ( 0.98963) ENFJ ( 0.99734) ENFJ ( 0.97496) ENFJ ( 0.95668) ENFJ ( 0.94375) ENFJ ( 0.96353) ENFJ ( 0.96132) ENFJ ( 0.96875)
ENFJ ( 0.97881) ENFJ ( 0.98116) ENFJ ( 0.98489) ENFJ ( 0.98997) ENFJ ( 0.95160) ENFJ ( 0.96957) ENFJ ( 0.97978) ENFJ ( 0.97338) ENFJ ( 0.88438) ENFJ ( 0.98116)
ENFJ ( 0.98213) ENFJ ( 0.98556) ENFJ ( 0.98431) ENFJ ( 0.99065) ENFJ ( 0.86693) ENFJ ( 0.93409) ENFJ ( 0.97368) ENFJ ( 0.99673) ENFJ ( 0.98537) ENFJ ( 0.98381)
ENFJ ( 0.97336) ENFJ ( 0.98865) ENFJ ( 0.97405) ENFJ ( 0.97382) ENFJ ( 0.98939) ENFJ ( 0.97452) ENFJ ( 0.97282) ENFJ ( 0.99186) ENFJ ( 0.98997) ENFJ ( 0.97983)
ENFJ ( 0.94865) ENFJ ( 0.98735) ENFJ ( 0.93697) ENFJ ( 0.97217) ENFJ ( 0.98630) ENFJ ( 0.95409) ENFJ ( 0.98241) ENFJ ( 0.88824) ENFJ ( 0.96373) ENFJ ( 0.98827)
ENFJ ( 0.85962) ENFJ ( 0.99530) ENFJ ( 0.97368) ENFJ ( 0.94098) ENFJ ( 0.96562) ENFJ ( 0.99288) ENFJ ( 0.99287) ENFJ ( 0.98635) ENFJ ( 0.98266) ENFJ ( 0.99075)
Loop 9
ENFJ ( 0.98943) ENFJ ( 1.00000) ENFJ ( 0.99984) ENFJ ( 0.99267) ENFJ ( 0.99984) ENFJ ( 0.99646) ENFJ ( 0.98096) ENFJ ( 0.99267) ENFJ ( 0.98788) ENFJ ( 0.98720)
ENFJ ( 0.96031) ENFJ ( 1.00000) ENFJ ( 0.97571) ENFJ ( 0.99235) ENFJ ( 0.99984) ENFJ ( 0.97647) ENFJ ( 0.99984) ENFJ ( 0.96955) ENFJ ( 0.99754) ENFJ ( 0.99673)
ENFJ ( 0.98630) ENFJ ( 1.00000) ENFJ ( 0.98926) ENFJ ( 0.98551) ENFJ ( 0.98431) ENFJ ( 0.99308) ENFJ ( 0.98087) ENFJ ( 0.97960) ENFJ ( 0.99734) ENFJ ( 0.98646)
ENFJ ( 0.97523) ENFJ ( 0.98759) ENFJ ( 0.99385) ENFJ ( 0.98646) ENFJ ( 0.95652) ENFJ ( 0.99916) ENFJ ( 0.97667) ENFJ ( 0.99984) ENFJ ( 0.97371) ENFJ ( 0.99503)
ENFJ ( 0.99984) ENFJ ( 0.97452) ENFJ ( 0.98646) ENFJ ( 0.99984) ENFJ ( 0.98997) ENFJ ( 0.99308) ENFJ ( 0.98049) ENFJ ( 0.99228) ENFJ ( 0.98537) ENFJ ( 0.99430)
ENFJ ( 0.92412) ENFJ ( 0.98646) ENFJ ( 0.97667) ENFJ ( 0.99984) ENFJ ( 0.94049) ENFJ ( 0.98210) ENFJ ( 0.98646) ENFJ ( 0.99673) ENFJ ( 0.97929) ENFJ ( 0.97213)
ENFJ ( 0.98646) ENFJ ( 0.99977) ENFJ ( 0.95709) ENFJ ( 0.98430) ENFJ ( 0.99206) ENFJ ( 0.99984) ENFJ ( 1.00000) ENFJ ( 0.96974) ENFJ ( 0.99984) ENFJ ( 1.00000)
ENFJ ( 0.99308) ENFJ ( 0.99411) ENFJ ( 0.97119) ENFJ ( 0.98646) ENFJ ( 0.98109) ENFJ ( 0.97762) ENFJ ( 0.99001) ENFJ ( 0.99984) ENFJ ( 0.96830) ENFJ ( 0.99936)
ENFJ ( 0.98837) ENFJ ( 0.99506) ENFJ ( 0.99430) ENFJ ( 0.99005) ENFJ ( 0.99916) ENFJ ( 0.97778) ENFJ ( 0.99889) ENFJ ( 0.99984) ENFJ ( 0.98346) ENFJ ( 0.99602)
ENFJ ( 0.99984) ENFJ ( 0.97358) ENFJ ( 0.98837) ENFJ ( 0.98527) ENFJ ( 0.99984) ENFJ ( 0.99936) ENFJ ( 0.99984) ENFJ ( 0.93354) ENFJ ( 0.99267) ENFJ ( 0.99065)
```
