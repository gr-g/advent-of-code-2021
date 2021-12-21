use std::collections::HashSet;
use std::collections::HashMap;

// Represent each rotation as a permutation of the axes 'p' and a
// reflection of the axes 'r', such that if a rotation (p, r) aligns a
// scanner to a new frame of reference, after the rotation an obsevation
//   [obs[0], obs[1], obs[2]]
// has coordinates
//   [obs[p[0]]*r[0], obs[p[1]]*r[1], obs[p[2]]*r[2]]
// in the new frame of reference.
const ROTATIONS: [([usize; 3], [i16; 3]); 24] = [
    ([0, 1, 2], [ 1,  1,  1]),
    ([0, 1, 2], [ 1, -1, -1]),
    ([0, 1, 2], [-1,  1, -1]),
    ([0, 1, 2], [-1, -1,  1]),
    ([1, 2, 0], [ 1,  1,  1]),
    ([1, 2, 0], [ 1, -1, -1]),
    ([1, 2, 0], [-1,  1, -1]),
    ([1, 2, 0], [-1, -1,  1]),
    ([2, 0, 1], [ 1,  1,  1]),
    ([2, 0, 1], [ 1, -1, -1]),
    ([2, 0, 1], [-1,  1, -1]),
    ([2, 0, 1], [-1, -1,  1]),
    ([0, 2, 1], [ 1,  1, -1]),
    ([0, 2, 1], [ 1, -1,  1]),
    ([0, 2, 1], [-1,  1,  1]),
    ([0, 2, 1], [-1, -1, -1]),
    ([2, 1, 0], [ 1,  1, -1]),
    ([2, 1, 0], [ 1, -1,  1]),
    ([2, 1, 0], [-1,  1,  1]),
    ([2, 1, 0], [-1, -1, -1]),
    ([1, 0, 2], [ 1,  1, -1]),
    ([1, 0, 2], [ 1, -1,  1]),
    ([1, 0, 2], [-1,  1,  1]),
    ([1, 0, 2], [-1, -1, -1]),
];

// Apply a rotation and a translation to the current frame of reference.
fn change_reference(rotation: &([usize; 3], [i16; 3]), translation: &[i16; 3], obs: &[i16; 3]) -> [i16; 3] {
    let (p, r) = rotation;
    [obs[p[0]]*r[0]-translation[0], obs[p[1]]*r[1]-translation[1], obs[p[2]]*r[2]-translation[2]]
}

fn solve(input: &str) -> (usize, i16) {
    let scanners: Vec<_> = input
        .split("\n\n")
        .map(|scanner| {
            scanner.lines().skip(1).map(|line| {
                let mut obs = line.split(',').map(|v| v.parse::<i16>().unwrap());
                [obs.next().unwrap(), obs.next().unwrap(), obs.next().unwrap()]
            }).collect::<Vec<_>>()
        })
        .collect();

    let n_scanners = scanners.len();

    // Rotation and translation to align each scanner.
    let mut scanners_info = vec![None; n_scanners];

    // Positions of the beacons.
    let mut beacons = HashSet::new();

    // Assume the frame of scanner[0] is the frame of reference.
    let (rotation, translation) = (([0, 1, 2], [1, 1, 1]), [0, 0, 0]);
    for obs in &scanners[0] {
        beacons.insert(change_reference(&rotation, &translation, obs));
    }
    scanners_info[0] = Some((rotation, translation));

    while !scanners_info.iter().all(|s| s.is_some()) {
        'scanner_loop: for s in 1..n_scanners {
            if scanners_info[s].is_some() {
                continue;
            }
            for rotation in &ROTATIONS {
                // If the rotation is correct and there is a translation t that
                // aligns the scanner to the frame of reference, then for a beacon
                // with position p:
                //
                //   change_reference(rotation, t, obs) = p
                //
                // The difference between the rotated observation and p gives
                // the implied translation t:
                //
                //   change_reference(rotation, p, obs) = t
                //
                // The same implied translation t should appear every time an
                // observation is matched against the corresponding beacon.
                let mut implied_translations = HashMap::new();
                for p in &beacons {
                    for obs in &scanners[s] {
                        *implied_translations.entry(change_reference(rotation, p, obs)).or_insert(0) += 1;
                    }
                }
                // Check if the same implied translation was recorded 12 of more times.
                for (t, count) in implied_translations {
                    if count >= 12 {
                        //println!("Identified scanner {}: rotation = {:?}, translation = {:?}", s, rotation, t);
                        for obs in &scanners[s] {
                            beacons.insert(change_reference(rotation, &t, obs));
                        }
                        scanners_info[s] = Some((*rotation, t));
                        continue 'scanner_loop;
                    }
                }
            }
        }
    }

    let mut pairwaise_distances = vec![];
    for i in 0..n_scanners-1 {
        for j in i+1..n_scanners {
            let t_i = scanners_info[i].unwrap().1;
            let t_j = scanners_info[j].unwrap().1;
            pairwaise_distances.push(
                (t_i[0]-t_j[0]).abs() + (t_i[1]-t_j[1]).abs() + (t_i[2]-t_j[2]).abs()
            );
        }
    }

    (beacons.len(), pairwaise_distances.into_iter().max().unwrap())
}

fn main() {
    let input = std::fs::read_to_string("input/19.txt").unwrap();
    let now = std::time::Instant::now();
    let s = solve(&input);
    println!("Solution: {:?}", s);
    println!("Time: {}ms", now.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example01() {
        assert_eq!(solve("\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"), (79, 3621));
    }
}
