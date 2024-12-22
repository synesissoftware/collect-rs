
use crate::containers::FrequencyMap;


// TODO: remove this and make generic
pub type ValueType = u64;

#[derive(Debug)]
pub struct DistributionAnalysis {
    count : isize,
    min : Option<ValueType>,
    max : Option<ValueType>,
    mean : Option<ValueType>,
    modes : Vec<ValueType>,
    p50 : Option<ValueType>,
    p90 : Option<ValueType>,
    p95 : Option<ValueType>,
    p98 : Option<ValueType>,
    p99 : Option<ValueType>,
    p99_9 : Option<ValueType>,
    p99_99 : Option<ValueType>,
}

// TODO: consider having only a single `Option<>` holding a `Percentiles` structure


// API functions

impl DistributionAnalysis {
}

// Mutating methods

impl DistributionAnalysis {
}

// Non-mutating methods

impl DistributionAnalysis {
    pub fn count(&self) -> isize {
        self.count
    }

    pub fn min(&self) -> Option<ValueType> {
        self.min
    }

    pub fn max(&self) -> Option<ValueType> {
        self.max
    }

    pub fn mean(&self) -> Option<ValueType> {
        self.mean
    }

    pub fn median(&self) -> Option<ValueType> {
        self.p50
    }

    pub fn modes(&self) -> &Vec<ValueType> {
        &self.modes
    }

    pub fn p50(&self) -> Option<ValueType> {
        self.p50
    }

    pub fn p90(&self) -> Option<ValueType> {
        self.p90
    }

    pub fn p95(&self) -> Option<ValueType> {
        self.p95
    }

    pub fn p98(&self) -> Option<ValueType> {
        self.p98
    }

    pub fn p99(&self) -> Option<ValueType> {
        self.p99
    }

    pub fn p99_9(&self) -> Option<ValueType> {
        self.p99_9
    }

    pub fn p99_99(&self) -> Option<ValueType> {
        self.p99_99
    }
}

// implementation

impl DistributionAnalysis {
}

// Trait implementations

impl From<&FrequencyMap<u64>> for DistributionAnalysis {
    /// T.B.C.
    ///
    /// # Note:
    /// This function assumes all counts are positive.
    fn from(value: &FrequencyMap<u64>) -> Self {

        let mut count = 0isize;
        let mut min = None;
        let mut max = None;
        let mut mean = None;
        let mut modes = vec![];
        let mut mode_count = 0;

        let mut p50 = None;
        let mut p90 = None;
        let mut p95 = None;
        let mut p98 = None;
        let mut p99 = None;
        let mut p99_9 = None;
        let mut p99_99 = None;

        let mut pairs = vec![];

        let mut total_count = 0usize;
        let mut total_sum = 0u128;

        for (&k, &n) in value.iter() {
            count += n;

            match &mut min {
                None => {
                    min = Some(k);
                },
                Some(v) => {
                    if *v > k {
                        *v = k;
                    }
                },
            };

            match &mut max {
                None => {
                    max = Some(k);
                },
                Some(v) => {
                    if *v < k {
                        *v = k;
                    }
                },
            };

            total_count += n as usize;
            total_sum += n as u128 * k as u128;

            pairs.push((k, n));
        }

        pairs.sort();

        if 0 != total_count {

            mean = Some((total_sum / total_count as u128) as u64);

            if 1 == total_count {

                modes.push(min.unwrap());
                p50 = min;
            } else {
                // now pass through again, with the expected counts

                let p50_count = (total_count + 1) / 2;
                let p90_count = (total_count + 1) * 9 / 10;
                let p95_count = (total_count + 1) * 19 / 20;
                let p98_count = (total_count + 1) * 49 / 50;
                let p99_count = (total_count + 1) * 99 / 100;
                let p99_9_count = (total_count + 1) * 999 / 1000;
                let p99_99_count = (total_count + 1) * 9999 / 10000;
                /*
                let p100_count = (total_count + 1) * 10000 / 10000;
                 */
                let mut cum_count = 0;

                for &(k, n) in &pairs {

                    if n > mode_count {
                        modes.clear();

                        mode_count = n;
                    }

                    if n >= mode_count {
                        modes.push(k);
                    }

                    cum_count += n as usize;

                    if p50.is_none() {
                        if cum_count >= p50_count {
                            p50 = Some(k);
                        }
                    }

                    if p90.is_none() {
                        if cum_count >= p90_count {
                            p90 = Some(k);
                        }
                    }


                    if p95.is_none() {
                        if cum_count >= p95_count {
                            p95 = Some(k);
                        }
                    }

                    if p98.is_none() {
                        if cum_count >= p98_count {
                            p98 = Some(k);
                        }
                    }

                    if p99.is_none() {
                        if cum_count >= p99_count {
                            p99 = Some(k);
                        }
                    }

                    if p99_9.is_none() {
                        if cum_count >= p99_9_count {
                            p99_9 = Some(k);
                        }
                    }

                    if p99_99.is_none() {
                        if cum_count >= p99_99_count {
                            p99_99 = Some(k);
                        }
                    }
                }
            }
        }

        Self {
            count,
            min,
            max,
            mean,
            modes,
            p50,
            p90,
            p95,
            p98,
            p99,
            p99_9,
            p99_99,
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::DistributionAnalysis;


    mod from_FrequencyMap {
        #![allow(non_snake_case)]

        use super::*;

        use crate::containers::FrequencyMap;


        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_EMPTY() {

            let fm = FrequencyMap::default();

            let da = DistributionAnalysis::from(fm);

            assert_eq!(0, da.count());

            assert_eq!(None, da.min());
            assert_eq!(None, da.max());
            assert_eq!(None, da.mean());
            assert_eq!(None, da.median());
            assert_eq!(0, da.modes().len());

            assert_eq!(None, da.p50());
            assert_eq!(None, da.p90());
            assert_eq!(None, da.p95());
            assert_eq!(None, da.p98());
            assert_eq!(None, da.p99());
            assert_eq!(None, da.p99_9());
            assert_eq!(None, da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_1_ELEMENT() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(1, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(101), da.max());
            assert_eq!(Some(101), da.mean());
            assert_eq!(Some(101), da.median());
            assert_eq!(1, da.modes().len());
            assert_eq!(101, da.modes()[0]);

            assert_eq!(Some(101), da.p50());
            assert_eq!(None, da.p90());
            assert_eq!(None, da.p95());
            assert_eq!(None, da.p98());
            assert_eq!(None, da.p99());
            assert_eq!(None, da.p99_9());
            assert_eq!(None, da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_2_ELEMENTS() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
                101,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(2, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(101), da.max());
            assert_eq!(Some(101), da.mean());
            assert_eq!(Some(101), da.median());
            assert_eq!(1, da.modes().len());
            assert_eq!(101, da.modes()[0]);


            assert_eq!(Some(101), da.p50());
            assert_eq!(Some(101), da.p90());
            assert_eq!(Some(101), da.p95());
            assert_eq!(Some(101), da.p98());
            assert_eq!(Some(101), da.p99());
            assert_eq!(Some(101), da.p99_9());
            assert_eq!(Some(101), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_2_ELEMENTS_INCREASING_UNIQUE_VALUES() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
                103,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(2, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(103), da.max());
            assert_eq!(Some(102), da.mean());
            assert_eq!(Some(101), da.median());
            assert_eq!(2, da.modes().len());
            assert_eq!(101, da.modes()[0]);
            assert_eq!(103, da.modes()[1]);

            assert_eq!(Some(101), da.p50());
            assert_eq!(Some(103), da.p90());
            assert_eq!(Some(103), da.p95());
            assert_eq!(Some(103), da.p98());
            assert_eq!(Some(103), da.p99());
            assert_eq!(Some(103), da.p99_9());
            assert_eq!(Some(103), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_3_ELEMENTS_ALL_SAME() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
                101,
                101,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(3, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(101), da.max());
            assert_eq!(Some(101), da.mean());
            assert_eq!(Some(101), da.median());
            assert_eq!(1, da.modes().len());
            assert_eq!(101, da.modes()[0]);

            assert_eq!(Some(101), da.p50());
            assert_eq!(Some(101), da.p90());
            assert_eq!(Some(101), da.p95());
            assert_eq!(Some(101), da.p98());
            assert_eq!(Some(101), da.p99());
            assert_eq!(Some(101), da.p99_9());
            assert_eq!(Some(101), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_3_ELEMENTS_INCREASING_UNIQUE_VALUES() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
                103,
                129,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(3, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(129), da.max());
            assert_eq!(Some(111), da.mean());
            assert_eq!(Some(103), da.median());
            assert_eq!(3, da.modes().len());
            assert_eq!(101, da.modes()[0]);
            assert_eq!(103, da.modes()[1]);
            assert_eq!(129, da.modes()[2]);

            assert_eq!(Some(103), da.p50());
            assert_eq!(Some(129), da.p90());
            assert_eq!(Some(129), da.p95());
            assert_eq!(Some(129), da.p98());
            assert_eq!(Some(129), da.p99());
            assert_eq!(Some(129), da.p99_9());
            assert_eq!(Some(129), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_1() {

            let fm = FrequencyMap::from([
                // insert list:
                101,
                102,
                103,
                104,
                105,
                106,
                107,
                108,
                109,
                110,
                111,
                112,
                113,
                114,
                115,
                116,
                117,
                118,
                119,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(19, da.count());

            assert_eq!(Some(101), da.min());
            assert_eq!(Some(119), da.max());
            assert_eq!(Some(110), da.mean());
            assert_eq!(Some(110), da.median());
            assert_eq!(19, da.modes().len());

            assert_eq!(Some(110), da.p50());
            assert_eq!(Some(118), da.p90());
            assert_eq!(Some(119), da.p95());
            assert_eq!(Some(119), da.p98());
            assert_eq!(Some(119), da.p99());
            assert_eq!(Some(119), da.p99_9());
            assert_eq!(Some(119), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_2() {

            let fm = FrequencyMap::from([
                // insert list:
                90,
                91,
                92,
                93,
                94,
                95,
                96,
                97,
                98,
                99,
                100,
                101,
                102,
                103,
                104,
                105,
                106,
                107,
                108,
                109,
                110,
                111,
                112,
                113,
                114,
                115,
                116,
                117,
                118,
                119,
                120,
                121,
                122,
                123,
                124,
                125,
                126,
                127,
                128,
                129,
                130,
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(41, da.count());

            assert_eq!(Some(90), da.min());
            assert_eq!(Some(130), da.max());
            assert_eq!(Some(110), da.mean());
            assert_eq!(Some(110), da.median());
            assert_eq!(41, da.modes().len());

            assert_eq!(Some(110), da.p50());
            assert_eq!(Some(126), da.p90());
            assert_eq!(Some(128), da.p95());
            assert_eq!(Some(130), da.p98());
            assert_eq!(Some(130), da.p99());
            assert_eq!(Some(130), da.p99_9());
            assert_eq!(Some(130), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_3() {

            let fm = FrequencyMap::from([
                // insert list:
                (500, 100),
                (501, 10),
            ]);

            let da = DistributionAnalysis::from(fm);

            assert_eq!(110, da.count());

            assert_eq!(Some(500), da.min());
            assert_eq!(Some(501), da.max());
            assert_eq!(Some(500), da.mean());
            assert_eq!(Some(500), da.median());
            assert_eq!(1, da.modes().len());
            assert_eq!(500, da.modes()[0]);

            assert_eq!(Some(500), da.p50());
            assert_eq!(Some(500), da.p90());
            assert_eq!(Some(501), da.p95());
            assert_eq!(Some(501), da.p98());
            assert_eq!(Some(501), da.p99());
            assert_eq!(Some(501), da.p99_9());
            assert_eq!(Some(501), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_IN_INCLUSIVE_RANGE_1_100() {

            let fm = FrequencyMap::from_iter((1..=100).into_iter());

            let da = DistributionAnalysis::from(fm);

            assert_eq!(100, da.count());

            assert_eq!(Some(1), da.min());
            assert_eq!(Some(100), da.max());
            assert_eq!(Some(50), da.mean());
            assert_eq!(Some(50), da.median());
            assert_eq!(100, da.modes().len());

            assert_eq!(Some(50), da.p50());
            assert_eq!(Some(90), da.p90());
            assert_eq!(Some(95), da.p95());
            assert_eq!(Some(98), da.p98());
            assert_eq!(Some(99), da.p99());
            assert_eq!(Some(100), da.p99_9());
            assert_eq!(Some(100), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_IN_INCLUSIVE_RANGE_1_1000() {

            let fm = FrequencyMap::from_iter((1..=1000).into_iter());

            let da = DistributionAnalysis::from(fm);

            assert_eq!(1000, da.count());

            assert_eq!(Some(1), da.min());
            assert_eq!(Some(1000), da.max());
            assert_eq!(Some(500), da.mean());
            assert_eq!(Some(500), da.median());
            assert_eq!(1000, da.modes().len());

            assert_eq!(Some(500), da.p50());
            assert_eq!(Some(900), da.p90());
            assert_eq!(Some(950), da.p95());
            assert_eq!(Some(980), da.p98());
            assert_eq!(Some(990), da.p99());
            assert_eq!(Some(999), da.p99_9());
            assert_eq!(Some(1000), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_IN_INCLUSIVE_RANGE_1_10000() {

            let fm = FrequencyMap::from_iter((1..=10000).into_iter());

            let da = DistributionAnalysis::from(fm);

            assert_eq!(10000, da.count());

            assert_eq!(Some(1), da.min());
            assert_eq!(Some(10000), da.max());
            assert_eq!(Some(5000), da.mean());
            assert_eq!(Some(5000), da.median());
            assert_eq!(10000, da.modes().len());

            assert_eq!(Some(5000), da.p50());
            assert_eq!(Some(9000), da.p90());
            assert_eq!(Some(9500), da.p95());
            assert_eq!(Some(9800), da.p98());
            assert_eq!(Some(9900), da.p99());
            assert_eq!(Some(9990), da.p99_9());
            assert_eq!(Some(9999), da.p99_99());
        }

        #[test]
        fn TEST_DistributionAnalysis_From_FrequencyMap_MANY_ELEMENTS_IN_INCLUSIVE_RANGE_1_100000() {

            let fm = FrequencyMap::from_iter((1..=100000).into_iter());

            let da = DistributionAnalysis::from(fm);

            assert_eq!(100000, da.count());

            assert_eq!(Some(1), da.min());
            assert_eq!(Some(100000), da.max());
            assert_eq!(Some(50000), da.mean());
            assert_eq!(Some(50000), da.median());
            assert_eq!(100000, da.modes().len());

            assert_eq!(Some(50000), da.p50());
            assert_eq!(Some(90000), da.p90());
            assert_eq!(Some(95000), da.p95());
            assert_eq!(Some(98000), da.p98());
            assert_eq!(Some(99000), da.p99());
            assert_eq!(Some(99900), da.p99_9());
            assert_eq!(Some(99990), da.p99_99());
        }
    }
}
