use std::collections::HashMap;
use std::fmt;

struct Player {
    id: i32,
    secret: f32,
    rands: Vec<f32>,
    poly: Option<Box<dyn Fn(f32) -> f32>>,
    shares: HashMap<i32, f32>,
    folded_share: f32,
}

impl fmt::Debug for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Player {{ id: {}, secret: {}, rands: {:?}, shares: {:?}, folded_share: {} }}",
            self.id, self.secret, self.rands, self.shares, self.folded_share
        )
    }
}

impl Player {
    fn new(id: i32, secret: f32, rands: Vec<f32>) -> Self {
        Self {
            id,
            secret,
            rands,
            poly: None,
            shares: HashMap::new(),
            folded_share: 0.0,
        }
    }

    fn make_poly(&mut self, k: usize) {
        let rands = self.rands.clone();
        let secret = self.secret;
        self.poly = Some(Box::new(move |x: f32| {
            let mut res = secret;
            for i in 0..(k - 1) {
                res += rands[i] * x.powf((i + 1) as f32);
            }
            res
        }));
        self.shares
            .insert(self.id, self.poly.as_ref().unwrap()(self.id as f32));
    }

    fn give_share(&self, opposite_id: i32) -> f32 {
        self.poly.as_ref().unwrap()(opposite_id as f32)
    }

    fn recieve_share(&mut self, opposite_player: &Player) {
        self.shares
            .insert(opposite_player.id, opposite_player.give_share(self.id));
    }

    fn fold_share(&mut self, method: impl Fn(&HashMap<i32, f32>) -> f32) {
        self.folded_share = method(&self.shares);
    }
}

// parts stands for participants
fn phis(parts: &[i32]) -> HashMap<i32, f32> {
    let mut res = HashMap::new();
    for (i, &p) in parts.iter().enumerate() {
        let p = p as f32;
        let mut r = 1.0;
        for (j, &q) in parts.iter().enumerate() {
            let q = q as f32;
            if i == j {
                continue;
            }

            if p == q {
                panic!("Invalid participants");
            }

            r *= -q / (p - q);
        }
        res.insert(p as i32, r);
    }
    res
}

fn add_simulation() {
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

    player1.fold_share(|shares| shares.values().sum::<f32>());
    player2.fold_share(|shares| shares.values().sum::<f32>());
    player3.fold_share(|shares| shares.values().sum::<f32>());

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

fn mul_simulation() {
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

    player1.fold_share(|shares| shares.values().product::<f32>());
    player2.fold_share(|shares| shares.values().product::<f32>());
    player3.fold_share(|shares| shares.values().product::<f32>());

    println!("p1: {:?}", player1);
    println!("p2: {:?}", player2);
    println!("p3: {:?}", player3);

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let i1: i32 = rng.gen_range(0..17);
    let i2: i32 = rng.gen_range(0..17);
    let i3: i32 = rng.gen_range(0..17);
    let mut player1_m = Player::new(1, player1.folded_share, vec![i1 as f32]);
    let mut player2_m = Player::new(2, player2.folded_share, vec![i2 as f32]);
    let mut player3_m = Player::new(3, player3.folded_share, vec![i3 as f32]);

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
            .sum::<f32>()
    });
    player2_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, v)| phs123.get(k).unwrap() * v)
            .sum::<f32>()
    });
    player3_m.fold_share(|shares| {
        shares
            .iter()
            .map(|(k, v)| phs123.get(k).unwrap() * v)
            .sum::<f32>()
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

fn main() {
    println!("Add simulation");
    add_simulation();
    println!("Mul simulation");
    mul_simulation();

    /* its result is:
    Add simulation
    p1: Player { id: 1, secret: 2, rands: [5.0], shares: {1: 7.0, 2: 7.0}, folded_share: 14 }
    p2: Player { id: 2, secret: 4, rands: [3.0], shares: {1: 12.0, 2: 10.0}, folded_share: 22 }
    p3: Player { id: 3, secret: 6, rands: [7.0], shares: {1: 17.0, 2: 13.0}, folded_share: 30 }
    [p1, p2] s_1 + s_2 = 6
    [p1, p3] s_1 + s_2 = 6
    [p2, p3] s_1 + s_2 = 6
    Mul simulation
    p1: Player { id: 1, secret: 2, rands: [5.0], shares: {2: 7.0, 1: 7.0}, folded_share: 49 }
    p2: Player { id: 2, secret: 4, rands: [3.0], shares: {2: 10.0, 1: 12.0}, folded_share: 120 }
    p3: Player { id: 3, secret: 6, rands: [7.0], shares: {1: 17.0, 2: 13.0}, folded_share: 221 }
    p1m: Player { id: 1, secret: 49, rands: [13.0], shares: {2: 123.0, 1: 62.0, 3: 222.0}, folded_share: 39 }
    p2m: Player { id: 2, secret: 120, rands: [3.0], shares: {1: 75.0, 2: 126.0, 3: 223.0}, folded_share: 70 }
    p3m: Player { id: 3, secret: 221, rands: [1.0], shares: {3: 224.0, 2: 129.0, 1: 88.0}, folded_share: 101 }
    [p1, p2] s_1 * s_2 = 8
    [p1, p3] s_1 * s_2 = 8
    [p2, p3] s_1 * s_2 = 8
        */
}
