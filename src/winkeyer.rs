#[repr(u8)]
#[derive(Clone,Debug)]
enum Command {
    Admin(AdminCommand) = 0x00,

    SidetoneControl(u8) = 0x01,

    SetSpeedWPM(u8) = 0x02,

    SetWeighting(u8) = 0x03,

    SetPTTLeadAndTail(u8, u8) = 0x04,

    SetSpeedPOT(u8, u8, u8) = 0x05,

    SetPaused(u8) = 0x06,

    GetSpeedPOT = 0x07,

    DropSerialInputBufferCharacter = 0x08,

    SetPincfg(u8) = 0x09,

    BufferClearBuffer = 0x0a,

    SetKeyDown(u8) = 0x0b,

    SetHighSpeedCW(u8) = 0x0c,

    SetSpeedFarnsworthWPM(u8) = 0x0d,

    SetKeyerMode(u8) = 0x0e,

    // From the official documentation:
    //
    // This command is used to load all of WK3’s operating parameters to be
    // loaded in one block transfer. The values are binary and must be loaded in
    // order.
    //
    // The values are exactly the same as those loaded for the individual
    // commands.
    //
    // The preferred time to issue this command is at reset just after the
    // interface has been opened.
    //
    // Do not issue this command while WK3 is transmitting.
    //
    // 1.) Mode Register
    // 2.) Speed in WPM
    // 3.) Sidetone Frequency
    // 4.) Weight
    // 5.) Lead-In Time
    // 6.) Tail Time
    // 7.) MinWPM
    // 8.) WPM Range
    // 9.) X2 Mode
    // 10) Key Compensation
    // 11) Farnsworth WPM
    // 12) Paddle Setpoint
    // 13) Dit/Dah Ratio
    // 14) Pin Configuration
    // 15) X1 Mode
    LoadSettings(Vec<u8>) = 0x0f, // Load Defaults

    SetKeyingExtendedFirstSend(u8) = 0x10,

    SetKeyingCompensation(u8) = 0x11,

    NoOp = 0x13, // Null Command

    DoKey(KeyInput) = 0x14,

    GetKeyerStatus = 0x15,

    SetInputBufferCursor(u8) = 0x16,

    SetKeyerDitDahRatio = 0x17,

    BufferDoPTT(u8) = 0x18,

    BufferAssertKey(u8) = 0x19,

    BufferSleep(u8) = 0x1a,

    BufferMergeLetters(u8, u8) = 0x1b,

    BufferSetSpeedWPM(u8) = 0x1c,

    BufferSetHighSpeedCW(u8) = 0x1d,

    BufferCancelSpeedChange = 0x1e,

    BufferedNoOp = 0x1f,

    Other(u8),
}

impl TryInto<u8> for Command {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Command::Other(x) => Ok(x),
            _ => Ok(self.discriminant()),
        }
    }
}

impl<'a> TryInto<Vec<u8>> for Command {
    type Error = ();

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            Command::Admin(ref admin_command) => {
                let mut cmd: Vec<u8> = vec![self.discriminant()];
                let mut admin_cmd: Vec<u8> = admin_command.clone().try_into().expect("build admin command bytes");

                cmd.append(&mut admin_cmd);

                Ok(cmd)
            }
            Command::SidetoneControl(_) => todo!(),
            Command::SetSpeedWPM(_) => todo!(),
            Command::SetWeighting(_) => todo!(),
            Command::SetPTTLeadAndTail(_, _) => todo!(),
            Command::SetSpeedPOT(_, _, _) => todo!(),
            Command::SetPaused(_) => todo!(),
            Command::GetSpeedPOT => Ok(vec![self.discriminant()]),
            Command::DropSerialInputBufferCharacter => Ok(vec![self.discriminant()]),
            Command::SetPincfg(_) => todo!(),
            Command::BufferClearBuffer => Ok(vec![self.discriminant()]),
            Command::SetKeyDown(_) => todo!(),
            Command::SetHighSpeedCW(_) => todo!(),
            Command::SetSpeedFarnsworthWPM(_) => todo!(),
            Command::SetKeyerMode(_) => todo!(),
            Command::LoadSettings(_) => todo!(),
            Command::SetKeyingExtendedFirstSend(_) => todo!(),
            Command::SetKeyingCompensation(_) => todo!(),
            Command::NoOp => Ok(vec![self.discriminant()]),
            Command::DoKey(key_input) => {
                Ok(vec![self.discriminant(), key_input.try_into().expect("key input")])
            },
            Command::GetKeyerStatus => Ok(vec![self.discriminant()]),
            Command::SetInputBufferCursor(_) => todo!(),
            Command::SetKeyerDitDahRatio => Ok(vec![self.discriminant()]),
            Command::BufferDoPTT(_) => todo!(),
            Command::BufferAssertKey(_) => todo!(),
            Command::BufferSleep(_) => todo!(),
            Command::BufferMergeLetters(_, _) => todo!(),
            Command::BufferSetSpeedWPM(_) => todo!(),
            Command::BufferSetHighSpeedCW(_) => todo!(),
            Command::BufferCancelSpeedChange => Ok(vec![self.discriminant()]),
            Command::BufferedNoOp => Ok(vec![self.discriminant()]),
            Command::Other(c) => Ok(vec![c]),
        }
    }
}

#[repr(u8)]
#[non_exhaustive]
#[derive(Clone,Copy,Debug)]
enum KeyInput {
    Release = 0x00,
    Dit = 0x01,
    Dah = 0x02,
    Both = 0x03, // implying this respects configured behavior (dit-dah, dah-dit, iambic handling)
}

impl TryInto<u8> for KeyInput {
    type Error = ();

    fn try_into(self) -> Result<u8, Self::Error> {
        Ok(self.discriminant())
    }
}

trait ProsignMapping
where
    Self: for<'a> TryFrom<&'a [u8], Error = ()>,
{
    fn prosign_ascii() -> char;
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Clone,Debug)]
enum AdminCommand {
    // 0: Calibrate For WK1 send <00><00> pause 100 mSec <FF>
    // Ignored by WK2 and WK3
    Calibrate = 0x0,

    // 1: Reset Resets the WK3 processor to the power up state. Do not send this as part of the
    // initialization sequence. Only send this if you want to do a cold reboot of WK3.
    ResetKeyer = 0x1,

    // 2: Host Open Upon power-up, the host interface is closed. To enable host mode, the PC host must
    // issue the Admin:open <00><02> command. Upon open, WK3 will respond by sending the
    // revision code back to the host. The host must wait for this return code before any other
    // commands or data can be sent to WK3. Upon open, WK1 mode is set.
    OpenHostConnection = 0x2,

    // 3: Host Close Use this command to disable the host interface. WK3 will return to standby mode after this
    // command is issued and standby settings will be restored.
    CloseHostConnection = 0x3,

    // 4: Echo Test Used to test the serial interface. The next character sent to WK3 after this command will
    // be echoed back to the host. <00><04><65> echoes 65 (letter a)
    EchoTest(u8) = 0x4,

    // 5: Paddle A2D Historical command not supported in WK3, always returns 0.

    // 6: Speed A2D Historical command not supported in WK3, always returns 0.

    // 7: Get Values Historical command not supported in WK3, always returns 0.

    // 8: Reserved K1EL Debug use only
    DebugInternal = 0x8,

    // 9: Get FW Major Rev Returns the major firmware revision, 31 for rev 31.03
    GetFirmwareMajorVersion = 0x9,

    // 10: Set WK1 Mode Disables pushbutton reporting
    SetReportingV1 = 0x10,

    // 11: Set WK2 Mode Enables pushbutton reporting, alternate WK status mode is selected.
    SetReportingV2 = 0x11,

    // 12: Dump EEPROM Dumps all 256 bytes of WK3’s internal EEPROM.
    DumpEEPROM = 0x12,

    // 13: Load EEPROM Download all 256 bytes of WK3’s internal EEPROM.
    LoadEEPROM(Vec<u8>) = 0x13,

    // 14: Send Message Command WK3 to send one of its internal messages.
    // The syntax is: <00><14><msg number> where msg number is 1 through 6
    SendMessageByID = 0x14,

    // 15: Load X1MODE Load mode extension register 1, WK1 does not support this register. Note that the
    // bit assignments of this register are different between WK2 and WK3 mode
    LoadExtensionR1 = 0x15,

    // 16: Firmware Update This command initiates an image upload. This feature is protected.
    FirmwareUpdate = 0x16,

    // 17: Set Low Baud Change serial comm. Baud Rate to 1200 (default)
    SetBaudRateLow = 0x17,

    // 18: Set High Baud Change serial comm. Baud Rate to 9600
    SetBaudRateHigh = 0x18,

    // 19: Set RTTY Mode Registers <00><19><P1><P2> Specify RTTY Operation Mode (WK3.1 only)
    SetRTTYRegisters = 0x19,

    // 20: Set WK3 Mode Enables WinKeyer 3 functions; expanded X1MODE and additional X2MODE register
    SetReportingV3 = 0x20,

    // 21: Read Back Vcc Return WK IC power supply voltage. This command returns a single byte which
    // can be converted to voltage: 26214/byte value = Voltage*100

    // 22: Load X2MODE Load mode extension register 2. This register is active on WK3 mode only.

    // 23: Get FW Minor Rev Returns the minor firmware revision, 03 for version 31.03
    GetFirmwareMinorVersion = 0x23,
    // 24: Get IC Type Returns the WK IC type 0x1 for SMT, 0x0 for DIP

    // 25: Set Sidetone Volume <00><24><n> where n =0x1 for low and n=0x4 for high
    SetSidetoneVolume(u8) = 0x24,
}

impl TryInto<Vec<u8>> for AdminCommand {
    type Error = ();

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        match self {
            AdminCommand::SetReportingV3 => Ok(vec![self.discriminant()]),
            AdminCommand::GetFirmwareMinorVersion => Ok(vec![self.discriminant()]),
            AdminCommand::SetSidetoneVolume(vol) => Ok(vec![self.discriminant(), vol]),
            AdminCommand::Calibrate => Ok(vec![self.discriminant()]),
            AdminCommand::ResetKeyer => Ok(vec![self.discriminant()]),
            AdminCommand::OpenHostConnection => Ok(vec![self.discriminant()]),
            AdminCommand::CloseHostConnection => Ok(vec![self.discriminant()]),
            AdminCommand::EchoTest(x) => Ok(vec![self.discriminant(), x]),
            AdminCommand::DebugInternal => Ok(vec![self.discriminant()]),
            AdminCommand::GetFirmwareMajorVersion => Ok(vec![self.discriminant()]),
            AdminCommand::SetReportingV1 => Ok(vec![self.discriminant()]),
            AdminCommand::SetReportingV2 => Ok(vec![self.discriminant()]),
            AdminCommand::DumpEEPROM => Ok(vec![self.discriminant()]),
            AdminCommand::LoadEEPROM(ref eeprom) => {
                let mut cmd = vec![self.discriminant()];
                cmd.append(&mut eeprom.clone());
                Ok(cmd)
            },
            AdminCommand::SendMessageByID  => Ok(vec![self.discriminant()]),
            AdminCommand::LoadExtensionR1  => Ok(vec![self.discriminant()]),
            AdminCommand::FirmwareUpdate   => Ok(vec![self.discriminant()]),
            AdminCommand::SetBaudRateLow   => Ok(vec![self.discriminant()]),
            AdminCommand::SetBaudRateHigh  => Ok(vec![self.discriminant()]),
            AdminCommand::SetRTTYRegisters => Ok(vec![self.discriminant()]),
        }
    }
}

impl Command {
    // https://doc.rust-lang.org/stable/std/mem/fn.discriminant.html
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl AdminCommand {
    // https://doc.rust-lang.org/stable/std/mem/fn.discriminant.html
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl KeyInput {
    // https://doc.rust-lang.org/stable/std/mem/fn.discriminant.html
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn admin_command() {
        let admin_command = Command::Admin(AdminCommand::EchoTest(0x88));

        let cmd: Vec<u8> = admin_command.try_into().expect("cmd bytes");

        assert_eq!(cmd, vec![0x00, 0x04, 0x88]);

        let admin_command = Command::Admin(AdminCommand::LoadEEPROM(vec![0xf0, 0xe0, 0xd0]));

        let cmd: Vec<u8> = admin_command.try_into().expect("cmd bytes");

        assert_eq!(cmd, vec![0x00, 0x13, 0xf0, 0xe0, 0xd0]);
    }

    #[test]
    fn key_input() {
        let key_command = Command::DoKey(KeyInput::Dah);

        let cmd: Vec<u8> = key_command.try_into().expect("cmd bytes");

        assert_eq!(cmd, vec![0x14, 0x02]);

    }
}
