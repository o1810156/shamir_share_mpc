use num_traits::{NumAssign, One, Zero};
use std::collections::HashMap;
use std::convert::From;
use std::fmt;

struct Player<T>
where
    T: NumAssign + From<u16> + fmt::Display + fmt::Debug + Clone + Copy + 'static,
{
    id: u16,
    secret: T,
    rands: Vec<T>,
    poly: Option<Box<dyn Fn(T) -> T>>,
    shares: HashMap<u16, T>,
    folded_share: T,
}

impl<T> fmt::Debug for Player<T>
where
    T: NumAssign + From<u16> + fmt::Display + fmt::Debug + Clone + Copy + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, secret: {}, rands: {:?}, shares: {:?}, folded_share: {} }}",
            self.id, self.secret, self.rands, self.shares, self.folded_share
        )
    }
}

impl<T> Player<T>
where
    T: NumAssign + From<u16> + fmt::Display + fmt::Debug + Clone + Copy + 'static,
{
    fn new(id: u16, secret: T, rands: Vec<T>) -> Self {
        Self {
            id,
            secret,
            rands,
            poly: None,
            shares: HashMap::new(),
            folded_share: T::zero(),
        }
    }

    fn make_poly(&mut self, k: usize) {
        let rands = self.rands.clone();
        let secret = self.secret;
        self.poly = Some(Box::new(move |x: T| {
            let mut res = secret;
            let mut xn = x;
            for i in 0..(k - 1) {
                res += rands[i] * xn;
                xn *= x;
            }
            res
        }));
        self.shares
            .insert(self.id, self.poly.as_ref().unwrap()(self.id.into()));
    }

    fn give_share(&self, opposite_id: u16) -> T {
        self.poly.as_ref().unwrap()(opposite_id.into())
    }

    fn recieve_share(&mut self, opposite_player: &Player<T>) {
        self.shares
            .insert(opposite_player.id, opposite_player.give_share(self.id));
    }

    fn fold_share(&mut self, method: impl Fn(&HashMap<u16, T>) -> T) {
        self.folded_share = method(&self.shares);
    }
}

// parts stands for participants
fn phis<T>(parts: &[u16]) -> HashMap<u16, T>
where
    T: NumAssign + From<u16> + fmt::Display + fmt::Debug + Clone + Copy + 'static,
{
    let mut res = HashMap::new();
    for (i, &p_ognl) in parts.iter().enumerate() {
        let p: T = p_ognl.into();
        let mut r = T::one();
        for (j, &q) in parts.iter().enumerate() {
            let q: T = q.into();
            if i == j {
                continue;
            }

            if p == q {
                panic!("Invalid participants");
            }

            r *= (T::zero() - q) / (p - q);
        }
        res.insert(p_ognl, r);
    }
    res
}

fn add_simulation_f64() {
    let mut player1 = Player::new(1, 2.0, vec![5.0]);
    let mut player2 = Player::new(2, 4.0, vec![3.0]);
    let mut player3 = Player::new(3, 6.0, vec![7.0]); // player3 is the helper

    player1.make_poly(2);
    player2.make_poly(2);

    // player1.recieve_share(&player1);
    player1.recieve_share(&player2);
    player2.recieve_share(&player1);
    // player2.recieve_share(&player2);
    player3.recieve_share(&player1);
    player3.recieve_share(&player2);

    let phs12 = phis(&[player1.id, player2.id]);
    let phs13 = phis(&[player1.id, player3.id]);
    let phs23 = phis(&[player2.id, player3.id]);

    player1.fold_share(|shares| shares.values().sum::<f64>());
    player2.fold_share(|shares| shares.values().sum::<f64>());
    player3.fold_share(|shares| shares.values().sum::<f64>());

    println!("p1: {:?}", player1);
    println!("p2: {:?}", player2);
    println!("p3: {:?}", player3);

    // p1, p2
    println!(
        "[p1, p2] s_1 + s_2 = {}",
        phs12.get(&player1.id).unwrap() * player1.folded_share
            + phs12.get(&player2.id).unwrap() * player2.folded_share
    );

    // p1, p3
    println!(
        "[p1, p3] s_1 + s_2 = {}",
        phs13.get(&player1.id).unwrap() * player1.folded_share
            + phs13.get(&player3.id).unwrap() * player3.folded_share
    );

    // p2, p3
    println!(
        "[p2, p3] s_1 + s_2 = {}",
        phs23.get(&player2.id).unwrap() * player2.folded_share
            + phs23.get(&player3.id).unwrap() * player3.folded_share
    );
}

fn mul_simulation_f64() {
    let mut player1 = Player::new(1, 2.0, vec![5.0]);
    let mut player2 = Player::new(2, 4.0, vec![3.0]);
    let mut player3 = Player::new(3, 6.0, vec![7.0]); // player3 is the helper

    player1.make_poly(2);
    player2.make_poly(2);

    // player1.recieve_share(&player1);
    player1.recieve_share(&player2);
    player2.recieve_share(&player1);
    // player2.recieve_share(&player2);
    player3.recieve_share(&player1);
    player3.recieve_share(&player2);

    player1.fold_share(|shares| shares.values().product::<f64>());
    player2.fold_share(|shares| shares.values().product::<f64>());
    player3.fold_share(|shares| shares.values().product::<f64>());

    println!("p1: {:?}", player1);
    println!("p2: {:?}", player2);
    println!("p3: {:?}", player3);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let i1: u16 = rng.gen_range(0..17);
    let i2: u16 = rng.gen_range(0..17);
    let i3: u16 = rng.gen_range(0..17);
    let mut player1_m = Player::new(1, player1.folded_share, vec![i1 as f64]);
    let mut player2_m = Player::new(2, player2.folded_share, vec![i2 as f64]);
    let mut player3_m = Player::new(3, player3.folded_share, vec![i3 as f64]);

    player1_m.make_poly(2);
    player2_m.make_poly(2);
    player3_m.make_poly(2);

    // player1_m.recieve_share(&player1_m);
    player1_m.recieve_share(&player2_m);
    player1_m.recieve_share(&player3_m);
    player2_m.recieve_share(&player1_m);
    // player2_m.recieve_share(&player2_m);
    player2_m.recieve_share(&player3_m);
    player3_m.recieve_share(&player1_m);
    player3_m.recieve_share(&player2_m);
    // player3_m.recieve_share(&player3_m);

    let phs123 = phis(&[player1.id, player2.id, player3.id]);

    player1_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, v)| phs123.get(k).unwrap() * v)
            .sum::<f64>()
    });
    player2_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, v)| phs123.get(k).unwrap() * v)
            .sum::<f64>()
    });
    player3_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, v)| phs123.get(k).unwrap() * v)
            .sum::<f64>()
    });

    println!("p1m: {:?}", player1_m);
    println!("p2m: {:?}", player2_m);
    println!("p3m: {:?}", player3_m);

    let phs12 = phis(&[player1.id, player2.id]);
    let phs13 = phis(&[player1.id, player3.id]);
    let phs23 = phis(&[player2.id, player3.id]);

    // p1, p2
    println!(
        "[p1, p2] s_1 * s_2 = {}",
        phs12.get(&player1_m.id).unwrap() * player1_m.folded_share
            + phs12.get(&player2_m.id).unwrap() * player2_m.folded_share
    );

    // p1, p3
    println!(
        "[p1, p3] s_1 * s_2 = {}",
        phs13.get(&player1_m.id).unwrap() * player1_m.folded_share
            + phs13.get(&player3_m.id).unwrap() * player3_m.folded_share
    );

    // p2, p3
    println!(
        "[p2, p3] s_1 * s_2 = {}",
        phs23.get(&player2_m.id).unwrap() * player2_m.folded_share
            + phs23.get(&player3_m.id).unwrap() * player3_m.folded_share
    );
}

use shamir_share::ModInt;
type M = ModInt<17>;

fn add_simulation() {
    let mut player1 = Player::new(1, M::new(2), vec![M::new(5)]);
    let mut player2 = Player::new(2, M::new(4), vec![M::new(3)]);
    let mut player3 = Player::new(3, M::new(6), vec![M::new(7)]); // player3 is the helper

    player1.make_poly(2);
    player2.make_poly(2);

    // player1.recieve_share(&player1);
    player1.recieve_share(&player2);
    player2.recieve_share(&player1);
    // player2.recieve_share(&player2);
    player3.recieve_share(&player1);
    player3.recieve_share(&player2);

    let phs12: HashMap<u16, M> = phis(&[player1.id, player2.id]);
    let phs13: HashMap<u16, M> = phis(&[player1.id, player3.id]);
    let phs23: HashMap<u16, M> = phis(&[player2.id, player3.id]);

    player1.fold_share(|shares| shares.values().fold(M::zero(), |acc, &e| acc + e));
    player2.fold_share(|shares| shares.values().fold(M::zero(), |acc, &e| acc + e));
    player3.fold_share(|shares| shares.values().fold(M::zero(), |acc, &e| acc + e));

    println!("p1: {:?}", player1);
    println!("p2: {:?}", player2);
    println!("p3: {:?}", player3);

    // p1, p2
    println!(
        "[p1, p2] s_1 + s_2 = {}",
        *phs12.get(&player1.id).unwrap() * player1.folded_share
            + *phs12.get(&player2.id).unwrap() * player2.folded_share
    );

    // p1, p3
    println!(
        "[p1, p3] s_1 + s_2 = {}",
        *phs13.get(&player1.id).unwrap() * player1.folded_share
            + *phs13.get(&player3.id).unwrap() * player3.folded_share
    );

    // p2, p3
    println!(
        "[p2, p3] s_1 + s_2 = {}",
        *phs23.get(&player2.id).unwrap() * player2.folded_share
            + *phs23.get(&player3.id).unwrap() * player3.folded_share
    );
}

fn mul_simulation() {
    let mut player1 = Player::new(1, M::new(2), vec![M::new(5)]);
    let mut player2 = Player::new(2, M::new(4), vec![M::new(3)]);
    let mut player3 = Player::new(3, M::new(6), vec![M::new(7)]); // player3 is the helper

    player1.make_poly(2);
    player2.make_poly(2);

    // player1.recieve_share(&player1);
    player1.recieve_share(&player2);
    player2.recieve_share(&player1);
    // player2.recieve_share(&player2);
    player3.recieve_share(&player1);
    player3.recieve_share(&player2);

    player1.fold_share(|shares| shares.values().fold(M::one(), |acc, &e| acc * e));
    player2.fold_share(|shares| shares.values().fold(M::one(), |acc, &e| acc * e));
    player3.fold_share(|shares| shares.values().fold(M::one(), |acc, &e| acc * e));

    println!("p1: {:?}", player1);
    println!("p2: {:?}", player2);
    println!("p3: {:?}", player3);

    /*
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let i1: u16 = rng.gen_range(0..17);
    let i2: u16 = rng.gen_range(0..17);
    let i3: u16 = rng.gen_range(0..17);
    */
    // ↓検算のため固定
    let i1: u16 = 7;
    let i2: u16 = 9;
    let i3: u16 = 11;
    let mut player1_m = Player::new(1, player1.folded_share, vec![i1.into()]);
    let mut player2_m = Player::new(2, player2.folded_share, vec![i2.into()]);
    let mut player3_m = Player::new(3, player3.folded_share, vec![i3.into()]);

    player1_m.make_poly(2);
    player2_m.make_poly(2);
    player3_m.make_poly(2);

    // player1_m.recieve_share(&player1_m);
    player1_m.recieve_share(&player2_m);
    player1_m.recieve_share(&player3_m);
    player2_m.recieve_share(&player1_m);
    // player2_m.recieve_share(&player2_m);
    player2_m.recieve_share(&player3_m);
    player3_m.recieve_share(&player1_m);
    player3_m.recieve_share(&player2_m);
    // player3_m.recieve_share(&player3_m);

    let phs123: HashMap<u16, M> = phis(&[player1.id, player2.id, player3.id]);

    player1_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, &v)| *phs123.get(k).unwrap() * v)
            .fold(M::zero(), |acc, e| acc + e)
    });
    player2_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, &v)| *phs123.get(k).unwrap() * v)
            .fold(M::zero(), |acc, e| acc + e)
    });
    player3_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, &v)| *phs123.get(k).unwrap() * v)
            .fold(M::zero(), |acc, e| acc + e)
    });

    println!("p1m: {:?}", player1_m);
    println!("p2m: {:?}", player2_m);
    println!("p3m: {:?}", player3_m);

    let phs12: HashMap<u16, M> = phis(&[player1.id, player2.id]);
    let phs13: HashMap<u16, M> = phis(&[player1.id, player3.id]);
    let phs23: HashMap<u16, M> = phis(&[player2.id, player3.id]);

    // p1, p2
    println!(
        "[p1, p2] s_1 * s_2 = {}",
        *phs12.get(&player1_m.id).unwrap() * player1_m.folded_share
            + *phs12.get(&player2_m.id).unwrap() * player2_m.folded_share
    );

    // p1, p3
    println!(
        "[p1, p3] s_1 * s_2 = {}",
        *phs13.get(&player1_m.id).unwrap() * player1_m.folded_share
            + *phs13.get(&player3_m.id).unwrap() * player3_m.folded_share
    );

    // p2, p3
    println!(
        "[p2, p3] s_1 * s_2 = {}",
        *phs23.get(&player2_m.id).unwrap() * player2_m.folded_share
            + *phs23.get(&player3_m.id).unwrap() * player3_m.folded_share
    );
}

fn main() {
    println!("Add simulation f64");
    add_simulation_f64();
    println!("Mul simulation f64");
    mul_simulation_f64();

    println!("==============================");

    println!("Add simulation Z_17");
    add_simulation();
    println!("Mul simulation Z_17");
    mul_simulation();
}

/* its result is:
Add simulation f64
p1: Player { id: 1, secret: 2, rands: [5.0], shares: {1: 7.0, 2: 7.0}, folded_share: 14 }
p2: Player { id: 2, secret: 4, rands: [3.0], shares: {1: 12.0, 2: 10.0}, folded_share: 22 }
p3: Player { id: 3, secret: 6, rands: [7.0], shares: {1: 17.0, 2: 13.0}, folded_share: 30 }
[p1, p2] s_1 + s_2 = 6
[p1, p3] s_1 + s_2 = 6
[p2, p3] s_1 + s_2 = 6
Mul simulation f64
p1: Player { id: 1, secret: 2, rands: [5.0], shares: {1: 7.0, 2: 7.0}, folded_share: 49 }
p2: Player { id: 2, secret: 4, rands: [3.0], shares: {2: 10.0, 1: 12.0}, folded_share: 120 }
p3: Player { id: 3, secret: 6, rands: [7.0], shares: {1: 17.0, 2: 13.0}, folded_share: 221 }
p1m: Player { id: 1, secret: 49, rands: [6.0], shares: {3: 223.0, 2: 126.0, 1: 55.0}, folded_share: 10 }
p2m: Player { id: 2, secret: 120, rands: [6.0], shares: {2: 132.0, 1: 61.0, 3: 225.0}, folded_share: 12 }
p3m: Player { id: 3, secret: 221, rands: [2.0], shares: {2: 138.0, 1: 67.0, 3: 227.0}, folded_share: 14 }
[p1, p2] s_1 * s_2 = 8
[p1, p3] s_1 * s_2 = 8
[p2, p3] s_1 * s_2 = 8
==============================
Add simulation Z_17
p1: Player { id: 1, secret: 2, rands: [ModInt { val: 5 }], shares: {1: ModInt { val: 7 }, 2: ModInt { val: 7 }}, folded_share: 14 }
p2: Player { id: 2, secret: 4, rands: [ModInt { val: 3 }], shares: {2: ModInt { val: 10 }, 1: ModInt { val: 12 }}, folded_share: 5 }
p3: Player { id: 3, secret: 6, rands: [ModInt { val: 7 }], shares: {2: ModInt { val: 13 }, 1: ModInt { val: 0 }}, folded_share: 13 }
[p1, p2] s_1 + s_2 = 6
[p1, p3] s_1 + s_2 = 6
[p2, p3] s_1 + s_2 = 6
Mul simulation Z_17
p1: Player { id: 1, secret: 2, rands: [ModInt { val: 5 }], shares: {1: ModInt { val: 7 }, 2: ModInt { val: 7 }}, folded_share: 15 }
p2: Player { id: 2, secret: 4, rands: [ModInt { val: 3 }], shares: {2: ModInt { val: 10 }, 1: ModInt { val: 12 }}, folded_share: 1 }
p3: Player { id: 3, secret: 6, rands: [ModInt { val: 7 }], shares: {2: ModInt { val: 13 }, 1: ModInt { val: 0 }}, folded_share: 0 }
p1m: Player { id: 1, secret: 15, rands: [ModInt { val: 7 }], shares: {2: ModInt { val: 10 }, 3: ModInt { val: 11 }, 1: ModInt { val: 5 }}, folded_share: 13 }
p2m: Player { id: 2, secret: 1, rands: [ModInt { val: 9 }], shares: {2: ModInt { val: 2 }, 1: ModInt { val: 12 }, 3: ModInt { val: 5 }}, folded_share: 1 }
p3m: Player { id: 3, secret: 0, rands: [ModInt { val: 11 }], shares: {1: ModInt { val: 2 }, 3: ModInt { val: 16 }, 2: ModInt { val: 11 }}, folded_share: 6 }
[p1, p2] s_1 * s_2 = 8
[p1, p3] s_1 * s_2 = 8
[p2, p3] s_1 * s_2 = 8
*/
