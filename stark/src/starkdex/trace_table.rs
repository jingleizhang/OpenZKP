use crate::TraceTable;
use ecc::Affine;
use primefield::FieldElement;
use starkdex::{PEDERSEN_POINTS, SHIFT_POINT};
use std::prelude::v1::*;
use u256::U256;

struct Row {
    hasher_a: HashRow,      // Columns 0-3
    hasher_b: HashRow,      // Columns 4-7
    hasher_c: FieldElement, // Column 8
    mystery:  FieldElement, // Column 9
}

struct HashRow {
    source: FieldElement,
    slope:  FieldElement,
    x:      FieldElement,
    y:      FieldElement,
}

fn get_trace_table() -> TraceTable {
    let num_columns = 10;
    let num_rows = 2usize.pow(15);
    let mut trace_table = TraceTable::new(num_rows, num_columns);

    trace_table
}

struct SigConfig {
    pub alpha:       FieldElement,
    pub beta:        FieldElement,
    pub shift_point: Point,
}

struct Point {
    pub x: FieldElement,
    pub y: FieldElement,
}

fn test_hasher_1(trace_table: &TraceTable, i: usize) {
    // state_transition/merkle_update/prev_authentication/hashes/ec_subset_sum/bit =
    // column3_row0 - (column3_row1 + column3_row1)
    let source_bit = &trace_table[(3, i)] - trace_table[(3, i + 1)].double();

    let shift_point = Point {
        x: FieldElement::ONE,
        y: FieldElement::ZERO,
    };

    let merkle_hash_points__x = FieldElement::ONE;
    let merkle_hash_points__y = FieldElement::ONE;

    if i % 256 != 255 {
        assert!(source_bit == FieldElement::ZERO || source_bit == FieldElement::ONE);
        if source_bit.is_zero() {
            assert_eq!(
                &trace_table[(1, i)] - &merkle_hash_points__y,
                &trace_table[(2, i)] * (&trace_table[(0, i)] - &merkle_hash_points__x)
            );
            assert_eq!(
                trace_table[(2, i)].square(),
                &trace_table[(0, i)] + &merkle_hash_points__x + &trace_table[(0, i + 1)]
            );
            assert_eq!(
                &trace_table[(1, i)] + &trace_table[(1, i + 1)],
                &trace_table[(2, i)] * (&trace_table[(0, i)] - &trace_table[(0, i + 1)]),
            );
        } else {
            assert_eq!(trace_table[(0, i)], trace_table[(0, i + 1)]);
            assert_eq!(trace_table[(1, i)], trace_table[(1, i + 1)]);
            assert_eq!(trace_table[(2, i)], FieldElement::ZERO);
        }
    }
    if (i % 256 == 0) && !(i % 512 == 256) {
        assert_eq!(trace_table[(0, i + 256)], trace_table[(0, i + 255)]);
        assert_eq!(trace_table[(1, i + 256)], trace_table[(1, i + 255)]);
    }
    if (i % 512 == 0) {
        assert_eq!(trace_table[(0, i)], shift_point.x);
        assert_eq!(trace_table[(1, i)], shift_point.y);
    }
    if (i % 256 == 251) || (i % 256 == 255) {
        assert!(trace_table[(3, i)].is_zero());
    }
    // state_transition/merkle_update/side_bit_extraction/bit_1 =
    // column6_row767 - (column6_row1279 + column6_row1279)
    let state_transition__merkle_update__side_bit_extraction__bit_1 =
        &trace_table[(6, i + 767)] - trace_table[(6, i + 767 + 512)].double();
    if (i % 512 == 0) && !(i % 16384 == 16384 / 32 * 31 || i % 16384 == 16384 / 16 * 15) {
        if state_transition__merkle_update__side_bit_extraction__bit_1.is_zero() {
            assert_eq!(trace_table[(0, i + 511)], trace_table[(3, i + 512)]);
        } else {
            assert_eq!(trace_table[(0, i + 511)], trace_table[(3, i + 768)]);
        }
    }
}

fn test_hasher_2(trace_table: &TraceTable, i: usize) {
    // state_transition/merkle_update/new_authentication/hashes/ec_subset_sum/bit =
    // column7_row0 - (column7_row1 + column7_row1)
    let source_bit = &trace_table[(7, i)] - trace_table[(7, i + 1)].double();

    let shift_point = Point {
        x: FieldElement::ONE,
        y: FieldElement::ZERO,
    };

    let merkle_hash_points__x = FieldElement::ONE;
    let merkle_hash_points__y = FieldElement::ONE;

    if i % 256 != 255 {
        assert!(source_bit == FieldElement::ZERO || source_bit == FieldElement::ONE);
        if source_bit.is_zero() {
            assert_eq!(trace_table[(4, i)], trace_table[(4, i + 1)]);
            assert_eq!(trace_table[(5, i)], trace_table[(5, i + 1)]);
            assert_eq!(trace_table[(6, i)], FieldElement::ZERO);
        } else {
            assert_eq!(
                &trace_table[(5, i)] - &merkle_hash_points__y,
                &trace_table[(6, i)] * (&trace_table[(4, i)] - &merkle_hash_points__x),
            );
            assert_eq!(
                trace_table[(6, i)].square(),
                &trace_table[(4, i)] + &merkle_hash_points__x + &trace_table[(4, i + 1)],
            );
            assert_eq!(
                &trace_table[(5, i)] + &trace_table[(5, i + 1)],
                &trace_table[(6, i)] * (&trace_table[(4, i)] - &trace_table[(4, i + 1)]),
            );
        }
    }
    if (i % 256 == 0) && !(i % 512 == 256) {
        assert_eq!(trace_table[(4, i + 256)], trace_table[(4, i + 255)]);
        assert_eq!(trace_table[(5, i + 256)], trace_table[(5, i + 255)]);
    }
    if (i % 512 == 0) {
        assert_eq!(trace_table[(4, i)], shift_point.x);
        assert_eq!(trace_table[(5, i)], shift_point.y);
    }
    // state_transition/merkle_update/side_bit_extraction/bit_1 =
    // column6_row767 - (column6_row1279 + column6_row1279)
    let state_transition__merkle_update__side_bit_extraction__bit_1 =
        &trace_table[(6, i + 767)] - trace_table[(6, i + 767 + 512)].double();
    if i % 512 == 0 && i / 512 % 32 != 31 && i / 512 % 32 != 30 {
        if state_transition__merkle_update__side_bit_extraction__bit_1.is_zero() {
            assert_eq!(trace_table[(4, i + 511)], trace_table[(7, i + 512)]);
        } else {
            assert_eq!(trace_table[(4, i + 511)], trace_table[(7, i + 768)]);
        }
    }
    if (i % 256 == 251) || (i % 256 == 255) {
        assert!(trace_table[(7, i)].is_zero());
    }
}

fn test_hasher_3(trace_table: &TraceTable, i: usize) {
    // hash_pool/hash/ec_subset_sum/bit =
    // column8_row3 - (column8_row7 + column8_row7)
    let source_bit = &trace_table[(8, i + 3)] - trace_table[(8, i + 7)].double();

    let shift_point = Point {
        x: FieldElement::ONE,
        y: FieldElement::ONE,
    };

    let hash_pool_points__y = FieldElement::ONE;
    let hash_pool_points__x = FieldElement::ONE;

    if (i % 4 == 0) && (i / 4 % 256 != 255) {
        assert!(source_bit == FieldElement::ZERO || source_bit == FieldElement::ONE);
        if source_bit.is_zero() {
            assert_eq!(trace_table[(8, i + 4)], trace_table[(8, i)]);
            assert_eq!(trace_table[(8, i + 6)], trace_table[(8, i + 2)]);
        } else {
            assert_eq!(
                &trace_table[(8, i + 2)] - hash_pool_points__y,
                &trace_table[(8, i + 1)] * (&trace_table[(8, i)] - &hash_pool_points__x),
            );
            assert_eq!(
                trace_table[(8, i + 1)].square(),
                &trace_table[(8, i)] + &hash_pool_points__x + &trace_table[(8, i + 4)],
            );
            assert_eq!(
                &trace_table[(8, i + 2)] + &trace_table[(8, i + 6)],
                &trace_table[(8, i + 1)] * (&trace_table[(8, i)] - &trace_table[(8, i + 4)]),
            );
        }
    }
    if (i % 1024 == 0) && !(i % 2048 == 2048 / 2) {
        assert_eq!(trace_table[(8, i + 1024)], trace_table[(8, i + 1020)]);
        assert_eq!(trace_table[(8, i + 1026)], trace_table[(8, i + 1022)]);
    }
    if (i % 2048 == 0) {
        assert_eq!(trace_table[(8, i)], shift_point.x);
        assert_eq!(trace_table[(8, i + 2)], shift_point.y);
    }
    if (i % 4096 == 0) {
        assert_eq!(trace_table[(8, i + 2044)], trace_table[(8, i + 2051)]);
    }

    if (i % 1024 == 4 * 251) || (i % 1024 == 4 * 255) {
        assert!(trace_table[(8, i + 3)].is_zero());
    }
}

fn test_trace_table() {
    let trace_table = get_trace_table();
    for i in 0..trace_table.num_rows() {
        test_hasher_1(&trace_table, i);
        test_hasher_2(&trace_table, i);
        test_hasher_3(&trace_table, i);

        let is_settlement = FieldElement::ZERO; // periodic column from public input?
        let is_modification = FieldElement::ZERO; // periodic column from public input

        let state_transition__merkle_update__side_bit_extraction__bit_1 = FieldElement::ONE;
        // state_transition/merkle_update/side_bit_extraction/bit_0 =
        // column6_row255 - (column6_row767 + column6_row767)
        let state_transition__merkle_update__side_bit_extraction__bit_0 = FieldElement::ZERO;
        let state_transition__merkle_update__prev_authentication__leaf_0 = FieldElement::ONE;
        let state_transition__merkle_update__prev_authentication__sibling_0 = FieldElement::ZERO;
        let state_transition__merkle_update__new_authentication__sibling_0 = FieldElement::ZERO;
        let initial_root = FieldElement::ONE;
        let sig_config = SigConfig {
            alpha:       FieldElement::ONE,
            beta:        FieldElement::ONE,
            shift_point: Point {
                x: FieldElement::ONE,
                y: FieldElement::ONE,
            },
        };
        let final_root = FieldElement::ONE;
        let state_transition__merkle_update__new_authentication__leaf_0 = FieldElement::ZERO;

        // These are in the oods values...!?!?
        // Is it possible to express a non-constant shift in the constraint system?
        // These appear to be the entry points into hashers 1 and 2.
        let column4_row_expr0 = FieldElement::NEGATIVE_ONE;
        let column4_row_expr1 = FieldElement::NEGATIVE_ONE;
        let column0_row_expr2 = FieldElement::NEGATIVE_ONE;
        let column0_row_expr0 = FieldElement::NEGATIVE_ONE;

        let trace_length = 0;
        let path_length = 256;

        if (i % 512 == 0) && !(i % 16384 == 16384 / 32 * 31) {
            assert_eq!(
                &state_transition__merkle_update__side_bit_extraction__bit_0
                    * &state_transition__merkle_update__side_bit_extraction__bit_0
                    - &state_transition__merkle_update__side_bit_extraction__bit_0,
                FieldElement::ZERO
            );
            assert_eq!(
                state_transition__merkle_update__prev_authentication__sibling_0
                    - state_transition__merkle_update__new_authentication__sibling_0,
                FieldElement::ZERO
            );
        }

        // some sort of hash is being computed here?
        if (i % 16384 == 0) {
            assert_eq!(
                state_transition__merkle_update__prev_authentication__leaf_0,
                trace_table[(8, i + 4092)]
            );
            assert_eq!(
                state_transition__merkle_update__new_authentication__leaf_0,
                trace_table[(8, i + 12284)]
            );
            assert_eq!(
                (FieldElement::ONE - &trace_table[(8, i + 1021)]) * &trace_table[(9, i + 16376)]
                    - &trace_table[(8, i + 3)],
                FieldElement::ZERO
            );
            assert_eq!(
                (FieldElement::ONE - &trace_table[(8, i + 1021)]) * &trace_table[(9, i + 16360)]
                    - &trace_table[(8, i + 1027)],
                FieldElement::ZERO
            );
            assert_eq!(
                (FieldElement::ONE - &trace_table[(8, i + 9213)]) * &trace_table[(9, i + 16376)]
                    - &trace_table[(8, i + 8195)],
                FieldElement::ZERO
            );
            assert_eq!(
                (FieldElement::ONE - &trace_table[(8, i + 9213)]) * &trace_table[(9, i + 16360)]
                    - &trace_table[(8, i + 9219)],
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 8196)] - &trace_table[(8, i + 11267)],
                FieldElement::ZERO
            );
            assert_eq!(trace_table[(9, i + 48)], sig_config.shift_point.x);
            assert_eq!(trace_table[(9, i + 8)], sig_config.shift_point.y);
            assert_eq!(
                &trace_table[(9, i + 24)] * &trace_table[(9, i + 16336)] - FieldElement::ONE,
                FieldElement::ZERO
            );
        }
        // these signify hash ends...
        if (i % 16384 == 16384 / 256 * 251) {
            assert_eq!(trace_table[(9, i + 24)], FieldElement::ZERO);
        }
        if (i % 16384 == 16384 / 256 * 255) {
            assert_eq!(trace_table[(9, i + 24)], FieldElement::ZERO);
        }
        if (i % 16384 == 16384 / 32 * path_length) {
            assert_eq!(trace_table[(6, i + 255)], FieldElement::ZERO);
        }

        // these two perform a range check somehow?
        // trace_table[(9, i + 4)] is being shifted over every 128 rows.
        if (i % 128 == 0) && !(i % 8192 == 8192 / 64 * 63) {
            // amounts_range_check/bit_0 = column9_row4 - (column9_row132 + column9_row132)
            let amounts_range_check__bit_0 =
                &trace_table[(9, i + 4)] - trace_table[(9, i + 132)].double();
            assert_eq!(
                &amounts_range_check__bit_0 * &amounts_range_check__bit_0
                    - &amounts_range_check__bit_0,
                FieldElement::ZERO
            );
        }
        if (i % 8192 == 8192 / 64 * 63) {
            assert_eq!(trace_table[(9, i + 4)], FieldElement::ZERO);
        }

        // Uses every 6 / 64 rows to compute n * g. Takes 256 repititions to finish.
        if (i % 64 == 0) && !(i % 64 * 256 == 64 * 255) {
            // we are using repeated squaring to compute x^n. here.
            // // sig_verify/exponentiate_key/bit = column9_row24 - (column9_row88 +
            // column9_row88)
            let sig_verify__exponentiate_key__bit = FieldElement::ONE;
            let sig_verify__exponentiate_key__bit_neg =
                FieldElement::ONE - &sig_verify__exponentiate_key__bit;

            // trace_table[(9, i + 24)] is the source or something?

            // trace_table[(9, i + 8)] is the x coordinate
            // trace_table[(9, i + 40)] is slope
            // trace_table[(9, i + 48)] is the y coordinate
            // here were are adding an elliptic curve points?
            assert!(
                sig_verify__exponentiate_key__bit == FieldElement::ZERO
                    || sig_verify__exponentiate_key__bit == FieldElement::ONE
            );
            if sig_verify__exponentiate_key__bit == FieldElement::ONE {
                assert_eq!(
                    &trace_table[(9, i + 8)] - &trace_table[(9, i + 32)],
                    &trace_table[(9, i + 40)] * (&trace_table[(9, i + 48)] - &trace_table[(9, i)])
                );
            } else {
                assert!(trace_table[(9, i + 40)].is_zero());
                assert_eq!(trace_table[(9, i + 112)], trace_table[(9, i + 48)]);
                assert_eq!(trace_table[(9, i + 72)], trace_table[(9, i + 8)]);
            }
            assert_eq!(
                trace_table[(9, i + 40)].square()
                    - &sig_verify__exponentiate_key__bit
                        * (&trace_table[(9, i + 48)]
                            + &trace_table[(9, i)]
                            + &trace_table[(9, i + 112)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &sig_verify__exponentiate_key__bit
                    * (&trace_table[(9, i + 8)] + &trace_table[(9, i + 72)])
                    - &trace_table[(9, i + 40)]
                        * (&trace_table[(9, i + 48)] - &trace_table[(9, i + 112)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 56)] * (&trace_table[(9, i + 48)] - &trace_table[(9, i)])
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
            // this is the slope when doubling an elliptic curve point.
            // sig_verify/doubling_key/x_squared = column9_row0 * column9_row0
            // trace_table[(9, i)] is the x coordinate
            // trace_table[(9, i + 32)] is the y coordinate
            // trace_table[(9, i + 96)] is the next y coordinate
            // trace_table[(9, i + 16)] is the slope
            let doubling_point_slope = &trace_table[(9, i + 16)];
            let sig_verify__doubling_key__x_squared = trace_table[(9, i)].square();
            assert_eq!(
                &sig_verify__doubling_key__x_squared
                    + &sig_verify__doubling_key__x_squared
                    + &sig_verify__doubling_key__x_squared
                    + &sig_config.alpha,
                (&trace_table[(9, i + 32)] + &trace_table[(9, i + 32)]) * doubling_point_slope
            );
            assert_eq!(
                trace_table[(9, i + 64)],
                doubling_point_slope.square() - &trace_table[(9, i)] - &trace_table[(9, i)],
            );
            assert_eq!(
                trace_table[(9, i + 96)],
                doubling_point_slope * (&trace_table[(9, i)] - &trace_table[(9, i + 64)])
                    - &trace_table[(9, i + 32)]
            );
        }

        // This usees the product from above to do something.
        if (i % 128 == 0) && !(i % 32768 == 32768 / 256 * 256) {
            let sig_verify__exponentiate_generator__bit = FieldElement::ONE;
            let sig_verify__exponentiate_generator__bit_neg =
                FieldElement::ONE - &sig_verify__exponentiate_generator__bit;

            let ecdsa_points__y = FieldElement::ONE;
            let ecdsa_points__x = FieldElement::ONE;

            assert!(
                sig_verify__exponentiate_generator__bit == FieldElement::ZERO
                    || sig_verify__exponentiate_generator__bit == FieldElement::ONE
            );

            assert_eq!(
                &sig_verify__exponentiate_generator__bit
                    * (&trace_table[(9, i + 36)] - ecdsa_points__y)
                    - &trace_table[(9, i + 100)] * (&trace_table[(9, i + 68)] - &ecdsa_points__x),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 100)] * &trace_table[(9, i + 100)]
                    - &sig_verify__exponentiate_generator__bit
                        * (&trace_table[(9, i + 68)]
                            + &ecdsa_points__x
                            + &trace_table[(9, i + 196)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &sig_verify__exponentiate_generator__bit
                    * (&trace_table[(9, i + 36)] + &trace_table[(9, i + 164)])
                    - &trace_table[(9, i + 100)]
                        * (&trace_table[(9, i + 68)] - &trace_table[(9, i + 196)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 84)] * (&trace_table[(9, i + 68)] - &ecdsa_points__x)
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
            assert_eq!(
                &sig_verify__exponentiate_generator__bit_neg
                    * (&trace_table[(9, i + 196)] - &trace_table[(9, i + 68)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &sig_verify__exponentiate_generator__bit_neg
                    * (&trace_table[(9, i + 164)] - &trace_table[(9, i + 36)]),
                FieldElement::ZERO
            );
        }

        // this has to do with verifying signatures.
        if (i % 32768 == 0) {
            assert_eq!(trace_table[(9, i + 68)], sig_config.shift_point.x);
            assert_eq!(trace_table[(9, i + 36)], sig_config.shift_point.y);
            assert_eq!(
                &trace_table[(9, i + 32676)]
                    - &trace_table[(9, i + 16328)]
                    - &trace_table[(9, i + 32724)]
                        * (&trace_table[(9, i + 32708)] - &trace_table[(9, i + 16368)]),
                FieldElement::ZERO
            );
            assert_eq!(
                trace_table[(9, i + 32724)].square(),
                &trace_table[(9, i + 32708)]
                    + &trace_table[(9, i + 16368)]
                    + &trace_table[(9, i + 16384)]
            );
            assert_eq!(
                &trace_table[(9, i + 32676)] + &trace_table[(9, i + 16416)]
                    - &trace_table[(9, i + 32724)]
                        * (&trace_table[(9, i + 32708)] - &trace_table[(9, i + 16384)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 32740)]
                    * (&trace_table[(9, i + 32708)] - &trace_table[(9, i + 16368)])
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 32712)] + &sig_config.shift_point.y
                    - &trace_table[(8, i + 3069)]
                        * (&trace_table[(9, i + 32752)] - &sig_config.shift_point.x),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(8, i + 3069)] * &trace_table[(8, i + 3069)]
                    - (&trace_table[(9, i + 32752)]
                        + &sig_config.shift_point.x
                        + &trace_table[(9, i + 24)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(8, i + 19453)]
                    * (&trace_table[(9, i + 32752)] - &sig_config.shift_point.x)
                    - FieldElement::ONE,
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(9, i + 20)] * &trace_table[(8, i + 11261)] - FieldElement::ONE,
                FieldElement::ZERO
            );
            assert_eq!(trace_table[(8, i + 27645)], trace_table[(9, i)].square());
            assert_eq!(
                trace_table[(9, i + 32)].square(),
                &trace_table[(9, i)] * &trace_table[(8, i + 27645)]
                    + &sig_config.alpha * &trace_table[(9, i)]
                    + &sig_config.beta
            );
        }
        if (i % 32768 == 32768 / 256 * 251) {
            assert_eq!(trace_table[(9, i + 20)].clone(), FieldElement::ZERO);
        }
        if (i % 32768 == 32768 / 256 * 255) {
            assert_eq!(trace_table[(9, i + 20)].clone(), FieldElement::ZERO);
        }
        if (i % 65536 == 0) {
            if is_settlement == FieldElement::ONE {
                // These are constants? from the public input or agreed upon?
                let vault_shift = FieldElement::ONE;
                let trade_shift = FieldElement::ZERO;
                let amount_shift = FieldElement::ZERO;

                assert_eq!(
                    &trace_table[(8, i + 7171)]
                        - (((&trace_table[(6, i + 255)] * vault_shift
                            + &trace_table[(6, i + 49407)])
                            * &amount_shift
                            + &trace_table[(9, i + 4)]) // this is the amount that is shifted over?
                            * &amount_shift
                            + &trace_table[(9, i + 32772)])
                            * trade_shift,
                    FieldElement::ZERO
                );
                assert_eq!(
                    &trace_table[(8, i + 3075)]
                        - &trace_table[(8, i + 11267)]
                        - (&trace_table[(8, i + 27651)] - &trace_table[(8, i + 19459)]),
                    FieldElement::ZERO
                );
                assert_eq!(
                    &trace_table[(8, i + 35843)]
                        - &trace_table[(8, i + 44035)]
                        - (&trace_table[(8, i + 60419)] - &trace_table[(8, i + 52227)]),
                    FieldElement::ZERO
                );
                assert_eq!(
                    &trace_table[(9, i + 4)]
                        - (&trace_table[(8, i + 3075)] - &trace_table[(8, i + 11267)]),
                    FieldElement::ZERO
                );
                assert_eq!(
                    &trace_table[(9, i + 32772)]
                        - (&trace_table[(8, i + 35843)] - &trace_table[(8, i + 44035)]),
                    FieldElement::ZERO
                );
                assert_eq!(trace_table[(8, i + 36867)], trace_table[(8, i + 8188)]);
                assert_eq!(trace_table[(8, i + 37891)], trace_table[(6, i + 16639)]);
                assert_eq!(trace_table[(8, i + 39939)], trace_table[(6, i + 33023)]);
                assert_eq!(trace_table[(8, i + 8188)], trace_table[(9, i + 20)]);
                assert_eq!(trace_table[(8, i + 40956)], trace_table[(9, i + 32788)]);

                assert_eq!(trace_table[(8, i + 4099)], trace_table[(9, i + 16360)]);
                assert_eq!(trace_table[(8, i + 4099)], trace_table[(9, i + 32744)]);
                assert_eq!(trace_table[(8, i + 5123)], trace_table[(9, i + 65512)]);
                assert_eq!(trace_table[(8, i + 5123)], trace_table[(9, i + 49128)]);

                assert_eq!(trace_table[(9, i)], trace_table[(9, i + 16376)]);
                assert_eq!(trace_table[(9, i)], trace_table[(9, i + 65528)]);
                assert_eq!(trace_table[(9, i + 32768)], trace_table[(9, i + 32760)]);
                assert_eq!(trace_table[(9, i + 32768)], trace_table[(9, i + 49144)]);
            }

            if is_modification == FieldElement::ONE {
                let boundary_vault_id = FieldElement::ONE; // periodic_column
                let boundary_base = FieldElement::ONE; // periodic_column this is a 0-1 column?
                let boundary_amount0 = FieldElement::ONE; // periodic_column
                let boundary_amount1 = FieldElement::ONE; // periodic_column
                let boundary_token = FieldElement::ONE; // periodic_column
                let boundary_key = FieldElement::ONE; // periodic_column

                // ??? no idea what this does. it probably initializes the hash though?
                assert_eq!(column4_row_expr0, column4_row_expr1);

                assert_eq!(&trace_table[(9, i + 16376)] * &boundary_base, boundary_key);
                assert_eq!(
                    &trace_table[(9, i + 16360)] * &boundary_base,
                    boundary_token
                );
                assert_eq!(
                    &trace_table[(8, i + 3075)] * &boundary_base,
                    boundary_amount0
                );
                assert_eq!(
                    &trace_table[(8, i + 11267)] * &boundary_base,
                    boundary_amount1
                );
                // this verifies that the hash is correct for the vault id.
                assert_eq!(
                    &trace_table[(6, i + 255)] * &boundary_base,
                    boundary_vault_id
                );
            }
        }
        if (i % 8192 == 0) {
            // 1021 is 0 or 1
            // 3075 is 0 if 1021 isn't
            // 5117 is 0 if 1021 isn't
            // if 1021 is 0, then one of 3075 or 5117 is 0.
            // I think these together imply exactly one of three is non-zero?
            assert_eq!(
                &trace_table[(8, i + 1021)] * (FieldElement::ONE - &trace_table[(8, i + 1021)]),
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(8, i + 1021)] * &trace_table[(8, i + 3075)],
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(8, i + 1021)] * &trace_table[(8, i + 5117)],
                FieldElement::ZERO
            );
            assert_eq!(
                &trace_table[(8, i + 3075)] * &trace_table[(8, i + 5117)]
                    - (FieldElement::ONE - &trace_table[(8, i + 1021)]),
                FieldElement::ZERO
            );
        }
        if (i == 0) {
            assert_eq!(column0_row_expr0 - initial_root, FieldElement::ZERO);
        }
        if (i == trace_length - 65536) {
            assert_eq!(&column4_row_expr1 - final_root, FieldElement::ZERO);
        }
        if (i % 16384 == 0) && !(i == trace_length - 65536 + 49152) {
            assert_eq!(&column4_row_expr0 - column0_row_expr2, FieldElement::ZERO);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starkware_inputs_consistent() {
        test_trace_table()
    }
}
