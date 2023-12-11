
// This is the default multiplier used by PCG for 64-bit state.
const MULTIPLIER: u64 = 6364136223846793005;

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Pcg32 {
    state: u64,
    increment: u64,
}

impl Pcg32 {
    /// Multi-step advance functions (jump-ahead, jump-back)
    ///
    /// The method used here is based on Brown, "Random Number Generation
    /// with Arbitrary Stride,", Transactions of the American Nuclear
    /// Society (Nov. 1994).  The algorithm is very similar to fast
    /// exponentiation.
    ///
    /// Even though delta is an unsigned integer, we can pass a
    /// signed integer to go backwards, it just goes "the long way round".
    ///
    /// Using this function is equivalent to calling `next_32()` `delta`
    /// number of times.
    #[allow(dead_code)]
    #[inline]
    pub fn advance(&mut self, delta: u64) {
        let mut acc_mult: u64 = 1;
        let mut acc_plus: u64 = 0;
        let mut cur_mult = MULTIPLIER;
        let mut cur_plus = self.increment;
        let mut mdelta = delta;

        while mdelta > 0 {
            if (mdelta & 1) != 0 {
                acc_mult = acc_mult.wrapping_mul(cur_mult);
                acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
            }
            cur_plus = cur_mult.wrapping_add(1).wrapping_mul(cur_plus);
            cur_mult = cur_mult.wrapping_mul(cur_mult);
            mdelta /= 2;
        }
        self.state = acc_mult.wrapping_mul(self.state).wrapping_add(acc_plus);
    }

    /// Construct an instance compatible with PCG seed and stream.
    ///
    /// Note that the highest bit of the `stream` parameter is discarded
    /// to simplify upholding internal invariants.
    ///
    /// Note that two generators with different stream parameters may be closely
    /// correlated.
    ///
    /// PCG specifies the following default values for both parameters:
    ///
    /// - `state = 0xcafef00dd15ea5e5`
    /// - `stream = 0xa02bdbf7bb3c0a7`
    // Note: stream is 1442695040888963407u64 >> 1
    pub fn new(state: u64, stream: u64) -> Self {
        // The increment must be odd, hence we discard one bit:
        let increment = (stream << 1) | 1;
        Pcg32::from_state_incr(state, increment)
    }

    #[inline]
    fn from_state_incr(state: u64, increment: u64) -> Self {
        let mut pcg = Pcg32 { state, increment };
        // Move away from initial value:
        pcg.state = pcg.state.wrapping_add(pcg.increment);
        pcg.step();
        pcg
    }

    #[inline]
    fn step(&mut self) {
        // prepare the LCG for the next round
        self.state = self
            .state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let state = self.state;
        self.step();

        // Output function XSH RR: xorshift high (bits), followed by a random rotate
        // Constants are for 64-bit state, 32-bit output
        const ROTATE: u32 = 59; // 64 - 5
        const XSHIFT: u32 = 18; // (5 + 32) / 2
        const SPARE: u32 = 27; // 64 - 32 - 5

        let rot = (state >> ROTATE) as u32;
        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;
        xsh.rotate_right(rot)
    }
}
/// more advance functions
impl Pcg32 {
    /// Implement `next_u64` via `next_u32`, little-endian order.
    pub fn next_u64(&mut self) -> u64 {
        // Use LE; we explicitly generate one value before the next.
        let x = u64::from(self.next_u32());
        let y = u64::from(self.next_u32());
        (y << 32) | x
    }

    pub fn random_range<T>(&mut self, low: T, high : T) -> T 
    where Self: Rando<T>
    {
        Rando::<T>::random_range(self, low, high)

    }
    pub fn random_range_i32(&mut self, start: i32, end : i32) -> i32 {
        (self.next_u32() as i32 % (end - start)) + start
    }
    pub fn random<T>(&mut self) -> T 
    where Self: Rando<T>
    {
        Rando::<T>::random(self)

    }
}

pub trait Rando<T> {
    fn random(&mut self) -> T;
    fn random_range(&mut self, low: T, high: T) -> T;
}

impl Rando<i32> for Pcg32 {
    fn random(&mut self) -> i32 {
        (self.next_u32() % i32::MAX as u32) as i32
    }

    fn random_range(&mut self, low: i32, high: i32) -> i32 {
        (self.next_u32() % i32::MAX as u32) as i32 % (high - low) + low
    }
}
impl Rando<u32> for Pcg32 {
    fn random(&mut self) -> u32 {
        self.next_u32()
     }
    
    fn random_range(&mut self, low: u32, high: u32) -> u32 {
        (self.next_u32() % (high - low)) + low
    }
}
impl Rando<f64> for Pcg32{
    fn random(&mut self) -> f64 {
        self.next_u32() as f64 / u32::MAX as f64
    }

    fn random_range(&mut self, low: f64, high: f64) -> f64 {
        let diff = high - low;
        low + ( diff * self.random::<f64>())
    }
}
 

