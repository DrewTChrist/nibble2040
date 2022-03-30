use keyberon::action::{k, l, m, Action, Action::*};
use keyberon::key_code::KeyCode::*;
#[allow(unused_macros)]

// Shift + KeyCode
macro_rules! s {
    ($k:ident) => {
        m(&[LShift, $k])
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CustomActions {
    Underglow,
    Bootloader,
}

pub const ENCODER_LEFT: (u8, u8) = (3, 14);  // arrow up
pub const ENCODER_RIGHT: (u8, u8) = (4, 14); // arrow down

const UNDERGLOW: Action<CustomActions> = Action::Custom(CustomActions::Underglow);
const BOOTLOADER: Action<CustomActions> = Action::Custom(CustomActions::Bootloader);
const COPY: Action<CustomActions> = m(&[LCtrl, C]);
const PASTE: Action<CustomActions> = m(&[LCtrl, V]);

#[rustfmt::skip]
pub static LAYERS: keyberon::layout::Layers<CustomActions> = &[
    /* QWERTY */
    /* 
        All Trans keys are placeholders to even out the layout
        All k(No) keys are functional
    */
    &[
        &[Trans, k(Escape),   k(Kb1),  k(Kb2),  k(Kb3), k(Kb4), k(Kb5),   k(Kb6), k(Kb7), k(Kb8),   k(Kb9),   k(Kb0),    k(Minus),    k(Equal),    k(BSpace), Trans],
        &[k(No), k(Tab),      k(Q),    k(W),    k(E),   k(R),   k(T),     k(Y),   k(U),   k(I),     k(O),     k(P),      k(LBracket), k(RBracket), k(Bslash), Trans],
        &[k(No), k(CapsLock), k(A),    k(S),    k(D),   k(F),   k(G),     k(H),   k(J),   k(K),     k(L),     k(SColon), k(Quote),    Trans,       k(Enter),  Trans],
        &[k(No), k(LShift),   k(Z),    k(X),    k(C),   k(V),   k(B),     k(N),   k(M),   k(Comma), k(Dot),   k(Slash),  k(LShift),   Trans,       k(Up),     Trans],
        &[k(No), k(LCtrl),    k(LGui), k(LAlt), Trans,  Trans,  k(Space), Trans,  Trans,  k(RAlt),  k(RCtrl), k(No),     k(Left),     Trans,       k(Down),   k(Right)],
    ] 
];
