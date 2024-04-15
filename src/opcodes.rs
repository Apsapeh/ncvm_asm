#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    NOP = 0,
    STOP = 1,
    RET = 2,
    IMOV = 3,
    LMOV = 4,
    FMOV = 5,
    DMOV = 6,
    IRCLR = 7,
    LRCLR = 8,
    FRCLR = 9,
    DRCLR = 10,
    ISR = 11,
    LSR = 12,
    ISMLD = 13,
    ISMST = 14,
    LSMLD = 15,
    LSMST = 16,
    FSMLD = 17,
    FSMST = 18,
    DSMLD = 19,
    DSMST = 20,
    POPI = 21,
    POPA = 22,
    IPUSH = 23,
    ISTLD = 24,
    ISTST = 25,
    LPUSH = 26,
    LSTLD = 27,
    LSTST = 28,
    FPUSH = 29,
    FSTLD = 30,
    FSTST = 31,
    DPUSH = 32,
    DSTLD = 33,
    DSTST = 34,
    ALLOC = 35,
    FREE = 36,
    HELD = 37,
    HEST = 38,
    IADD = 39,
    ISUB = 40,
    IMULT = 41,
    IDIV = 42,
    IMOD = 43,
    IINC = 44,
    IDEC = 45,
    LADD = 46,
    LSUB = 47,
    LMULT = 48,
    LDIV = 49,
    LMOD = 50,
    LINC = 51,
    LDEC = 52,
    FADD = 53,
    FSUB = 54,
    FMULT = 55,
    FDIV = 56,
    FINC = 57,
    FDEC = 58,
    DADD = 59,
    DSUB = 60,
    DMULT = 61,
    DDIV = 62,
    DINC = 63,
    DDEC = 64,
    LTOI = 65,
    FTOI = 66,
    DTOI = 67,
    ITOL = 68,
    FTOL = 69,
    DTOL = 70,
    ITOF = 71,
    LTOF = 72,
    DTOF = 73,
    ITOD = 74,
    LTOD = 75,
    FTOD = 76,
    JMP = 77,
    IJEZ = 78,
    IJNZ = 79,
    IJEQ = 80,
    IJNQ = 81,
    IJML = 82,
    IJEL = 83,
    IJMG = 84,
    IJEG = 85,
    LJEZ = 86,
    LJNZ = 87,
    LJEQ = 88,
    LJNQ = 89,
    LJML = 90,
    LJEL = 91,
    LJMG = 92,
    LJEG = 93,
    FJEZ = 94,
    FJNZ = 95,
    FJEQ = 96,
    FJNQ = 97,
    FJML = 98,
    FJEL = 99,
    FJMG = 100,
    FJEG = 101,
    DJEZ = 102,
    DJNZ = 103,
    DJEQ = 104,
    DJNQ = 105,
    DJML = 106,
    DJEL = 107,
    DJMG = 108,
    DJEG = 109,
    L_CALL = 110,
}

struct CommandMapNode {
    key: &'static str,
    val: Opcode
}

impl CommandMapNode {
    pub const fn new(key: &'static str, val: Opcode) -> CommandMapNode {
        CommandMapNode {key, val}
    }
}

static COMMANDS: [CommandMapNode; 111] = [
    CommandMapNode::new("nop", Opcode::NOP),
    CommandMapNode::new("stop", Opcode::STOP),
    CommandMapNode::new("ret", Opcode::RET),
    CommandMapNode::new("imov", Opcode::IMOV),
    CommandMapNode::new("lmov", Opcode::LMOV),
    CommandMapNode::new("fmov", Opcode::FMOV),
    CommandMapNode::new("dmov", Opcode::DMOV),
    CommandMapNode::new("irclr", Opcode::IRCLR),
    CommandMapNode::new("lrclr", Opcode::LRCLR),
    CommandMapNode::new("frclr", Opcode::FRCLR),
    CommandMapNode::new("drclr", Opcode::DRCLR),
    CommandMapNode::new("isr", Opcode::ISR),
    CommandMapNode::new("lsr", Opcode::LSR),
    CommandMapNode::new("ismld", Opcode::ISMLD),
    CommandMapNode::new("ismst", Opcode::ISMST),
    CommandMapNode::new("lsmld", Opcode::LSMLD),
    CommandMapNode::new("lsmst", Opcode::LSMST),
    CommandMapNode::new("fsmld", Opcode::FSMLD),
    CommandMapNode::new("fsmst", Opcode::FSMST),
    CommandMapNode::new("dsmld", Opcode::DSMLD),
    CommandMapNode::new("dsmst", Opcode::DSMST),
    CommandMapNode::new("popi", Opcode::POPI),
    CommandMapNode::new("popa", Opcode::POPA),
    CommandMapNode::new("ipush", Opcode::IPUSH),
    CommandMapNode::new("istld", Opcode::ISTLD),
    CommandMapNode::new("istst", Opcode::ISTST),
    CommandMapNode::new("lpush", Opcode::LPUSH),
    CommandMapNode::new("lstld", Opcode::LSTLD),
    CommandMapNode::new("lstst", Opcode::LSTST),
    CommandMapNode::new("fpush", Opcode::FPUSH),
    CommandMapNode::new("fstld", Opcode::FSTLD),
    CommandMapNode::new("fstst", Opcode::FSTST),
    CommandMapNode::new("dpush", Opcode::DPUSH),
    CommandMapNode::new("dstld", Opcode::DSTLD),
    CommandMapNode::new("dstst", Opcode::DSTST),
    CommandMapNode::new("alloc", Opcode::ALLOC),
    CommandMapNode::new("free", Opcode::FREE),
    CommandMapNode::new("held", Opcode::HELD),
    CommandMapNode::new("hest", Opcode::HEST),
    CommandMapNode::new("iadd", Opcode::IADD),
    CommandMapNode::new("isub", Opcode::ISUB),
    CommandMapNode::new("imult", Opcode::IMULT),
    CommandMapNode::new("idiv", Opcode::IDIV),
    CommandMapNode::new("imod", Opcode::IMOD),
    CommandMapNode::new("iinc", Opcode::IINC),
    CommandMapNode::new("idec", Opcode::IDEC),
    CommandMapNode::new("ladd", Opcode::LADD),
    CommandMapNode::new("lsub", Opcode::LSUB),
    CommandMapNode::new("lmult", Opcode::LMULT),
    CommandMapNode::new("ldiv", Opcode::LDIV),
    CommandMapNode::new("lmod", Opcode::LMOD),
    CommandMapNode::new("linc", Opcode::LINC),
    CommandMapNode::new("ldec", Opcode::LDEC),
    CommandMapNode::new("fadd", Opcode::FADD),
    CommandMapNode::new("fsub", Opcode::FSUB),
    CommandMapNode::new("fmult", Opcode::FMULT),
    CommandMapNode::new("fdiv", Opcode::FDIV),
    CommandMapNode::new("finc", Opcode::FINC),
    CommandMapNode::new("fdec", Opcode::FDEC),
    CommandMapNode::new("dadd", Opcode::DADD),
    CommandMapNode::new("dsub", Opcode::DSUB),
    CommandMapNode::new("dmult", Opcode::DMULT),
    CommandMapNode::new("ddiv", Opcode::DDIV),
    CommandMapNode::new("dinc", Opcode::DINC),
    CommandMapNode::new("ddec", Opcode::DDEC),
    CommandMapNode::new("ltoi", Opcode::LTOI),
    CommandMapNode::new("ftoi", Opcode::FTOI),
    CommandMapNode::new("dtoi", Opcode::DTOI),
    CommandMapNode::new("itol", Opcode::ITOL),
    CommandMapNode::new("ftol", Opcode::FTOL),
    CommandMapNode::new("dtol", Opcode::DTOL),
    CommandMapNode::new("itof", Opcode::ITOF),
    CommandMapNode::new("ltof", Opcode::LTOF),
    CommandMapNode::new("dtof", Opcode::DTOF),
    CommandMapNode::new("itod", Opcode::ITOD),
    CommandMapNode::new("ltod", Opcode::LTOD),
    CommandMapNode::new("ftod", Opcode::FTOD),
    CommandMapNode::new("jmp", Opcode::JMP),
    CommandMapNode::new("ijez", Opcode::IJEZ),
    CommandMapNode::new("ijnz", Opcode::IJNZ),
    CommandMapNode::new("ijeq", Opcode::IJEQ),
    CommandMapNode::new("ijnq", Opcode::IJNQ),
    CommandMapNode::new("ijml", Opcode::IJML),
    CommandMapNode::new("ijel", Opcode::IJEL),
    CommandMapNode::new("ijmg", Opcode::IJMG),
    CommandMapNode::new("ijeg", Opcode::IJEG),
    CommandMapNode::new("ljez", Opcode::LJEZ),
    CommandMapNode::new("ljnz", Opcode::LJNZ),
    CommandMapNode::new("ljeq", Opcode::LJEQ),
    CommandMapNode::new("ljnq", Opcode::LJNQ),
    CommandMapNode::new("ljml", Opcode::LJML),
    CommandMapNode::new("ljel", Opcode::LJEL),
    CommandMapNode::new("ljmg", Opcode::LJMG),
    CommandMapNode::new("ljeg", Opcode::LJEG),
    CommandMapNode::new("fjez", Opcode::FJEZ),
    CommandMapNode::new("fjnz", Opcode::FJNZ),
    CommandMapNode::new("fjeq", Opcode::FJEQ),
    CommandMapNode::new("fjnq", Opcode::FJNQ),
    CommandMapNode::new("fjml", Opcode::FJML),
    CommandMapNode::new("fjel", Opcode::FJEL),
    CommandMapNode::new("fjmg", Opcode::FJMG),
    CommandMapNode::new("fjeg", Opcode::FJEG),
    CommandMapNode::new("djez", Opcode::DJEZ),
    CommandMapNode::new("djnz", Opcode::DJNZ),
    CommandMapNode::new("djeq", Opcode::DJEQ),
    CommandMapNode::new("djnq", Opcode::DJNQ),
    CommandMapNode::new("djml", Opcode::DJML),
    CommandMapNode::new("djel", Opcode::DJEL),
    CommandMapNode::new("djmg", Opcode::DJMG),
    CommandMapNode::new("djeg", Opcode::DJEG),
    CommandMapNode::new("l_call", Opcode::L_CALL),
];

impl Opcode {
    pub fn from_str(opcode: &str) -> Option<Opcode> {
        for c in &COMMANDS {
            if c.key == opcode {
                return Some(c.val)
            }
        };
        None
    }
}