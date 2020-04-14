use super::opcode::*;
use super::environment::*;
use super::operators::*;
use super::registers::*;

// 16 bit ADD opcodes
pub fn build_add_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD HL, {:?}", rr),
        cycles: 11, // IX or IY: 15
        action: Box::new(move |env: &mut Environment| {
            let aa = env.get_index_value();
            let bb = env.get_reg16(rr);
            let vv = operator_add16(env, aa, bb);
            env.set_reg16(Reg16::HL, vv);
        })
    }
}

pub fn build_adc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADC HL, {:?}", rr),
        cycles: 15,
        action: Box::new(move |env: &mut Environment| {
            let aa = env.get_index_value(); // This will always be HL.
            let bb = env.get_reg16(rr);
            let vv = operator_adc16(env, aa, bb);
            env.state.reg.set16(Reg16::HL, vv);
        })
    }
}

pub fn build_sbc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("SBC HL, {:?}", rr),
        cycles: 15,
        action: Box::new(move |env: &mut Environment| {
            let aa = env.get_index_value(); // This will always be HL.
            let bb = env.get_reg16(rr);
            let vv = operator_sbc16(env, aa, bb);
            env.state.reg.set16(Reg16::HL, vv);
        })
    }
}


// INC, DEC opcodes
pub fn build_inc_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("INC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23, IXH/IXL,IYH,IYL: 8
        action: Box::new(move |env: &mut Environment| {
            env.load_displacement(r);

            let a = env.get_reg(r);
            let v = operator_inc(env, a);
            env.set_reg(r, v);
        })
    }
}

pub fn build_dec_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("DEC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23, IXH/IXL,IYH,IYL: 8
        action: Box::new(move |env: &mut Environment| {
            env.load_displacement(r);

            let a = env.get_reg(r);
            let v = operator_dec(env, a);
            env.set_reg(r, v);
        })
    }
}

pub fn build_inc_dec_rr(rr: Reg16, inc: bool) -> Opcode {
    let delta = if inc {1} else {-1 as i16 as u16};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {:?}", mnemonic, rr),
        cycles: 6, // IX, IY: 10
        action: Box::new(move |env: &mut Environment| {
            let mut v = env.get_reg16(rr);
            v = v.wrapping_add(delta);
            env.set_reg16(rr, v);
            // Note: flags not affected on the 16 bit INC and DEC
        })
    }    
}    

// Misc. opcodes
pub fn build_neg() -> Opcode {
    Opcode {
        name: "NEG".to_string(),
        cycles: 8,
        action: Box::new(move |env: &mut Environment| {
            let b = env.state.reg.get_a();
            let v = operator_sub(env, 0, b);
            env.state.reg.set_a(v);
        })
    }
}

pub fn build_daa() -> Opcode {
    Opcode {
        name: "NEG".to_string(),
        cycles: 4,
        action: Box::new(move |env: &mut Environment| {
            // See TUZD-4.7
            let a = env.state.reg.get_a();
            let hi = a >> 4;
            let lo = a & 0xf;

            let nf = env.state.reg.get_flag(Flag::N);
            let cf = env.state.reg.get_flag(Flag::C);
            let hf = env.state.reg.get_flag(Flag::H);

            let lo6 = hf || (lo > 9);
            let hi6 = cf || (hi > 9) || (hi == 9 && lo > 9);
            let diff = if lo6 {6} else {0}
                + if hi6 {6<<4} else {0};
            let new_a = if nf {
                a.wrapping_sub(diff)
            } else {
                a.wrapping_add(diff)
            };

            let new_hf = (!nf && lo > 9) || (nf && hf && lo < 6);
            let new_cf = hi6;

            env.state.reg.set_a(new_a);
            env.state.reg.update_sz53p_flags(new_a);
            env.state.reg.put_flag(Flag::H, new_hf);
            env.state.reg.put_flag(Flag::C, new_cf);
            // N unchanged
        })
    }
}
