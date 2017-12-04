mod day1;
mod day2;
mod day3;
mod grid;
mod day4;

#[cfg(test)]
mod tests {
    #[test]
    fn solve_day1() {
        use day1::solve;
        let input = "9513446799636685297929646689682997114316733445451534532351778534251427172168183621874641711534917291674333857423799375512628489423332297538215855176592633692631974822259161766238385922277893623911332569448978771948316155868781496698895492971356383996932885518732997624253678694279666572149831616312497994856288871586777793459926952491318336997159553714584541897294117487641872629796825583725975692264125865827534677223541484795877371955124463989228886498682421539667224963783616245646832154384756663251487668681425754536722827563651327524674183443696227523828832466473538347472991998913211857749878157579176457395375632995576569388455888156465451723693767887681392547189273391948632726499868313747261828186732986628365773728583387184112323696592536446536231376615949825166773536471531487969852535699774113163667286537193767515119362865141925612849443983484245268194842563154567638354645735331855896155142741664246715666899824364722914296492444672653852387389477634257768229772399416521198625393426443499223611843766134883441223328256883497423324753229392393974622181429913535973327323952241674979677481518733692544535323219895684629719868384266425386835539719237716339198485163916562434854579365958111931354576991558771236977242668756782139961638347251644828724786827751748399123668854393894787851872256667336215726674348886747128237416273154988619267824361227888751562445622387695218161341884756795223464751862965655559143779425283154533252573949165492138175581615176611845489857169132936848668646319955661492488428427435269169173654812114842568381636982389224236455633316898178163297452453296667661849622174541778669494388167451186352488555379581934999276412919598411422973399319799937518713422398874326665375216437246445791623283898584648278989674418242112957668397484671119761553847275799873495363759266296477844157237423239163559391553961176475377151369399646747881452252547741718734949967752564774161341784833521492494243662658471121369649641815562327698395293573991648351369767162642763475561544795982183714447737149239846151871434656618825566387329765118727515699213962477996399781652131918996434125559698427945714572488376342126989157872118279163127742349";

        assert_eq!(solve(input, 1), 1343);
        assert_eq!(solve(input, input.len() / 2), 1274);
    }

    #[test]
    fn solve_day2() {
        use day2::{solve, row_data_min_max, row_data_evenly_divisible};
        let input = "
            1364	461	1438	1456	818	999	105	1065	314	99	1353	148	837	590	404	123
            204	99	235	2281	2848	3307	1447	3848	3681	963	3525	525	288	278	3059	821
            280	311	100	287	265	383	204	380	90	377	398	99	194	297	399	87
            7698	2334	7693	218	7344	3887	3423	7287	7700	2447	7412	6147	231	1066	248	208
            3740	837	4144	123	155	2494	1706	4150	183	4198	1221	4061	95	148	3460	550
            1376	1462	73	968	95	1721	544	982	829	1868	1683	618	82	1660	83	1778
            197	2295	5475	2886	2646	186	5925	237	3034	5897	1477	196	1778	3496	5041	3314
            179	2949	3197	2745	1341	3128	1580	184	1026	147	2692	212	2487	2947	3547	1120
            460	73	52	373	41	133	671	61	634	62	715	644	182	524	648	320
            169	207	5529	4820	248	6210	255	6342	4366	5775	5472	3954	3791	1311	7074	5729
            5965	7445	2317	196	1886	3638	266	6068	6179	6333	229	230	1791	6900	3108	5827
            212	249	226	129	196	245	187	332	111	126	184	99	276	93	222	56
            51	592	426	66	594	406	577	25	265	578	522	57	547	65	564	622
            215	2092	1603	1001	940	2054	245	2685	206	1043	2808	208	194	2339	2028	2580
            378	171	155	1100	184	937	792	1436	1734	179	1611	1349	647	1778	1723	1709
            4463	4757	201	186	3812	2413	2085	4685	5294	5755	2898	200	5536	5226	1028	180
            ";

        assert_eq!(solve(input, &row_data_min_max), 53460);
        assert_eq!(solve(input, &row_data_evenly_divisible), 282);
    }

    #[test]
    fn solve_day3() {
        use day3::{solve, solve_star_two};

        assert_eq!(solve(312051), 430);
        assert_eq!(solve_star_two(312051), 312453);
    }

    #[test]
    fn solve_day4() {
        use std::fs::File;
        use std::io::Read;

        use day4::{solve, UniquenessValidator, AnagramValidator};

        let mut input = String::new();
        let mut f = File::open("day4.txt").expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        assert_eq!(solve::<UniquenessValidator>(&input), 451);
        assert_eq!(solve::<AnagramValidator>(&input), 223);
    }
}
