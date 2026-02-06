use super::AsciiArt;
use colored::Color;

pub const AVAILABLE_LOGOS: &[&str] = &[
    "arch",
    "artix",
    "debian",
    "ubuntu",
    "fedora",
    "centos",
    "rhel",
    "opensuse",
    "gentoo",
    "void",
    "nixos",
    "alpine",
    "manjaro",
    "endeavouros",
    "pop",
    "mint",
    "elementary",
    "zorin",
    "kali",
    "parrot",
    "slackware",
    "linux",
];

pub fn get_logo(distro: &str) -> AsciiArt {
    match distro {
        "arch" | "archlinux" => arch(),
        "artix" | "artixlinux" => artix(),
        "debian" => debian(),
        "ubuntu" => ubuntu(),
        "fedora" => fedora(),
        "centos" => centos(),
        "rhel" | "redhat" => rhel(),
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => opensuse(),
        "gentoo" => gentoo(),
        "void" | "voidlinux" => void_linux(),
        "nixos" => nixos(),
        "alpine" => alpine(),
        "manjaro" => manjaro(),
        "endeavouros" => endeavouros(),
        "pop" | "pop_os" | "pop!_os" => pop_os(),
        "mint" | "linuxmint" => mint(),
        "elementary" | "elementaryos" => elementary(),
        "zorin" | "zorinos" => zorin(),
        "kali" => kali(),
        "parrot" | "parrotos" => parrot(),
        "slackware" => slackware(),
        _ => linux(),
    }
}

pub fn get_small_logo(distro: &str) -> AsciiArt {
    match distro {
        "arch" | "archlinux" => arch_small(),
        "debian" => debian_small(),
        "ubuntu" => ubuntu_small(),
        "fedora" => fedora_small(),
        "gentoo" => gentoo_small(),
        "void" | "voidlinux" => void_small(),
        "nixos" => nixos_small(),
        "manjaro" => manjaro_small(),
        _ => linux_small(),
    }
}

fn arch() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                   -`                 ",
            "                  .o+`                ",
            "                 `ooo/                ",
            "                `+oooo:               ",
            "               `+oooooo:              ",
            "               -+oooooo+:             ",
            "             `/:-:++oooo+:            ",
            "            `/++++/+++++++:           ",
            "           `/++++++++++++++:          ",
            "          `/+++ooooooooooooo/`        ",
            "         ./ooosssso++osssssso+`       ",
            "        .oossssso-````/ossssss+`      ",
            "       -osssssso.      :ssssssso.     ",
            "      :osssssss/        osssso+++.    ",
            "     /ossssssss/        +ssssooo/-    ",
            "   `/ossssso+/:-        -:/+osssso+-  ",
            "  `+sso+:-`                 `.-/+oso: ",
            " `++:.                           `-/+/",
            " .`                                 `.",
        ],
        colors: vec![Color::Cyan, Color::Blue],
        width: 38,
    }
}

fn arch_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "      /\\      ",
            "     /  \\     ",
            "    /\\   \\    ",
            "   /      \\   ",
            "  /   ,,   \\  ",
            " /   |  |  -\\ ",
            "/_-''    ''-_\\",
        ],
        colors: vec![Color::Cyan],
        width: 14,
    }
}

fn artix() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                   '                  ",
            "                  'o'                 ",
            "                 'ooo'                ",
            "                'ooxoo'               ",
            "               'ooxxxoo'              ",
            "              'oookkxxoo'             ",
            "             'oiaborxxxoo'            ",
            "            'ooxxddsiiabboo'          ",
            "            ':oxddsiiiiiioxoo'        ",
            "               'ioaboraborab.'        ",
            "          ':ooo;i]oaborabarboo'       ",
            "         'oooiiiioboaborabaarbb'      ",
            "        'ooxddsiiiiobbobobarbbbbb'    ",
            "       'oooiiibbbbbobbobobbbbbbbbb'   ",
            "      'oooiib]obobobobbbbbbbbbbbbbb'  ",
            "     'oooxddsobobobbbbbbbbbbbbbbbbb'  ",
            "    'ioabar]obbobobbbbbbbbbbbbbbbbbbb'",
            "   ':ob]oiob]obbbbbbbbbbbbbbbbbbbbbbbb",
            "       ''      'bbbbbbbbb''           ",
        ],
        colors: vec![Color::Cyan, Color::Blue],
        width: 38,
    }
}

fn debian() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "       _,met$$$$$gg.          ",
            "    ,g$$$$$$$$$$$$$$$P.       ",
            "  ,g$$P\"     \"\"\"Y$$.\".        ",
            " ,$$P'              `$$$.     ",
            "',$$P       ,ggs.     `$$b:   ",
            "`d$$'     ,$P\"'   .    $$$    ",
            " $$P      d$'     ,    $$P    ",
            " $$:      $$.   -    ,d$$'    ",
            " $$;      Y$b._   _,d$P'      ",
            " Y$$.    `.`\"Y$$$$P\"'         ",
            " `$$b      \"-.__              ",
            "  `Y$$                        ",
            "   `Y$$.                      ",
            "     `$$b.                    ",
            "       `Y$$b.                 ",
            "          `\"Y$b._             ",
            "              `\"\"\"            ",
        ],
        colors: vec![Color::Red],
        width: 30,
    }
}

fn debian_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "  _____  ",
            " /  __ \\ ",
            "|  /    |",
            "|  \\___- ",
            "-_       ",
            "  --_    ",
        ],
        colors: vec![Color::Red],
        width: 9,
    }
}

fn ubuntu() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "            .-/+oossssoo+/-.           ",
            "        `:+ssssssssssssssssss+:`       ",
            "      -+ssssssssssssssssssyyssss+-     ",
            "    .ossssssssssssssssssdMMMNysssso.   ",
            "   /ssssssssssshdmmNNmmyNMMMMhssssss/  ",
            "  +ssssssssshmydMMMMMMMNddddyssssssss+ ",
            " /sssssssshNMMMyhhyyyyhmNMMMNhssssssss/",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss",
            "ossyNMMMNyMMhsssssssssssssshmmmhsssssso",
            "ossyNMMMNyMMhsssssssssssssshmmmhsssssso",
            "+sssshhhyNMMNyssssssssssssyNMMMysssssss",
            ".ssssssssdMMMNhsssssssssshNMMMdssssssss",
            " /sssssssshNMMMyhhyyyyhdNMMMNhssssssss/",
            "  +sssssssssdmydMMMMMMMMddddyssssssss+ ",
            "   /ssssssssssshdmNNNNmyNMMMMhssssss/  ",
            "    .ossssssssssssssssssdMMMNysssso.   ",
            "      -+sssssssssssssssssyyyssss+-     ",
            "        `:+ssssssssssssssssss+:`       ",
            "            .-/+oossssoo+/-.           ",
        ],
        colors: vec![Color::Red, Color::White],
        width: 40,
    }
}

fn ubuntu_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "         _  ",
            "     ---(_) ",
            " _/  ---  \\ ",
            "(_) |   |   ",
            " \\  --- _/  ",
            "     ---(_) ",
        ],
        colors: vec![Color::Red],
        width: 12,
    }
}

fn fedora() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "             .',;::::;,'.             ",
            "         .';:cccccccccccc:;,.         ",
            "      .;cccccccccccccccccccccc;.      ",
            "    .:cccccccccccccccccccccccccc:.    ",
            "  .;ccccccccccccc;.:dddl:.;ccccccc;.  ",
            " .:ccccccccccccc;OWMKOOXMWd;ccccccc:. ",
            ".:ccccccccccccc;KMMc;cc;xMMc:ccccccc:.",
            ",cccccccccccccc;MMM.;cc;;WW::cccccccc,",
            ":cccccccccccccc;MMM.;cccccccccccccccc:",
            ":ccccccc;oxOOOo;MMM0telegrameeeseecc:",
            "cccccc:0telegramMMMMMMK]cc]cccccccccccc",
            "cccccc;c]ccccccAnchor.c]ccc]cccccccc:",
            ":ccccc;ccc]cccc]c]cccccccccc;cccccccc:",
            ":ccccc;ccc]ccc]ccccccccccccc;cccccccc:",
            ":cccccccc]cccc]ccc]cccccccccc;ccccccc:",
            ":cccccc;cccc]cccccccccccccccc;ccccccc:",
            " :ccc]c;ccc;cccc]cccccccccccc;cccccc: ",
            "  ':c;ccc;ccccccccccccccccccc;ccccc:  ",
            "     ':;ccccccccccccccccccccccc;:'    ",
            "        '::cccccccccccccc:::'         ",
        ],
        colors: vec![Color::Blue, Color::White],
        width: 38,
    }
}

fn fedora_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "      ____   ",
            "     /    \\\\ ",
            " ___|  f   | ",
            "|        __| ",
            "|___    |    ",
            "    |___|    ",
        ],
        colors: vec![Color::Blue],
        width: 13,
    }
}

fn centos() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                 ..                   ",
            "               .PLTJ.                 ",
            "              <><><><>                ",
            "     GY://telegramANCHOR::::://ABCDEF ",
            " cdEFGHIJKtelegram telegram:IJKLMNOPQ ",
            " abcd:::::MNOPQRS telegram:OPQRSTUV   ",
            "        .QRSTUV. telegram:STUVWXYZ    ",
            "           <><>  telegram telegram    ",
            "         ..VWXYZ..  :  :              ",
            "       .PTRSVWXYZ--.    :             ",
            "      <><><><><><>      :             ",
            "FIKPS::WPTRSVWXYZ::::::FGHIJK         ",
            " MQRWZ:::::::FGHIJK:::PQRSTUW         ",
            "       :::::OPQRSTUWV:::::            ",
            "            .TRSVWXYZ.                ",
            "               <><>                   ",
            "                ..                    ",
        ],
        colors: vec![Color::Yellow, Color::Green, Color::Blue, Color::Magenta],
        width: 38,
    }
}

fn rhel() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "           .MMM..:MMMMMMM              ",
            "          MMMMMMMMMMMMMMMMMM           ",
            "          MMMMMMMMMMMMMMMMMMMM.        ",
            "         MMMMMMMMMMMMMMMMMMMMMM        ",
            "        ,MMMMMMMMMMMMMMMMMMMMMM:       ",
            "        MMMMMMMMMMMMMMMMMMMMMMMM       ",
            "  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM       ",
            " MMMMMM    `MMMMMMMMMMMMMMMMMMMM.      ",
            "MMMMMMMM      MMMMMMMMMMMMMMMMMM .     ",
            "MMMMMMMMM.       `MMMMMMMMMMMMM' MM.   ",
            "MMMMMMMMMMM.                     MMMM  ",
            "`MMMMMMMMMMMMM.                 ,MMMMM.",
            " `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM",
            "    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM ",
            "      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM  ",
            "         MMMMMMMMMMMMMMMMMMMMMMMMMM    ",
            "            `MMMMMMMMMMMMMMMMMM'       ",
            "                ``MMMMMMMMM''          ",
        ],
        colors: vec![Color::Red],
        width: 40,
    }
}

fn opensuse() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "            .;ldkO0000Okdl;.            ",
            "        .;d00telegramtelegram00d;.      ",
            "      .d00:                    :00d.    ",
            "    .d0l'       .loddol.        'l0d.   ",
            "   .0Pd'     ':loooooooo:.        dP0.  ",
            "  .0KKKl.  ,oooooooooooooo:   .lKKK0.   ",
            " .0KKKKKd.,oooooooooooooooo:.dKKKKK0.   ",
            " 0KKKKKKK;ooooooooooooooooo;KKKKKKK0    ",
            " 0KKKKKKK;ooooooooooooooooo;KKKKKKK0    ",
            " 0KKKKKKK;ooooooooooooooooo;KKKKKKK0    ",
            " 0KKKKKKK;ooooooooooooooooo;KKKKKKK0    ",
            " 0KKKKKKK;ooooooooooooooooo;KKKKKKK0    ",
            " 0KKKKKKKd.;ooooooooooooo;.dKKKKKKK0    ",
            "  0KKKKKKKKo..;looool;..oKKKKKKKK0      ",
            "   0KKKKKKKKKKkl;,;;lkKKKKKKKKKK0       ",
            "    0KKKKKKKKKKKKKKKKKKKKKKKKKK0        ",
            "      :KKKKKKKKKKKKKKKKKKKKd:           ",
            "        :0KKKKKKKKKKd:                  ",
            "            '''''                       ",
        ],
        colors: vec![Color::Green],
        width: 42,
    }
}

fn gentoo() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "         -/oyddmdhs+:.              ",
            "     -odNMMMMMMMMNNmhy+-`           ",
            "   -yNMMMMMMMMMMMNNNmmdhy+-         ",
            " `omMMMMMMMMMMMMNmdmmmmddhhy/`      ",
            " omMMMMMMMMMMMNhhyyyohmdddhhhdo`    ",
            ".ydMMMMMMMMMMdoooyhshmmddhhhhdm+`   ",
            " ydMMMMMMMMMdooooooooohmmddhhhhdm+` ",
            "  odmMMMMMMMMdoo  oooooooommmddhhdm+",
            "   :ydMMMMMMMMMdooooooooooohmdhhhhdm",
            "    `:odMMMMMMMMNdooooooooooodmdhhdm",
            "      `:+ydNMMMMMMMNmdooo+oooohmhho ",
            "        `:/+oyhNMMMMMMNdyoooosmhy:  ",
            "           `:+oooyhNMMMMNNs+ohho.   ",
            "             `-/+oyhNMMMMNmhho.     ",
            "                `./+shNNNmho:       ",
            "                    `.:++-          ",
        ],
        colors: vec![Color::Magenta, Color::White],
        width: 36,
    }
}

fn gentoo_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            " _-----_  ",
            "(       \\ ",
            "\\    0   \\",
            " \\        )",
            " /      _/",
            "(     _-  ",
            "\\____-    ",
        ],
        colors: vec![Color::Magenta],
        width: 10,
    }
}

fn void_linux() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                __.;=====;.__                 ",
            "            _.=+==++=++=+=+===;.              ",
            "             -=+++=+===+=+=+++++=_            ",
            "        .     -=:``     `--==googol-          ",
            "       _vi,    `googol.telegramtelegram-      ",
            "      .telegramtelegram-Googol...googol-      ",
            "      ,googolGOOGOLGOOGOL-                    ",
            "    ,GOOGOLGOOGOLGOOGOL.                      ",
            "     _GOOGOL.=googol-GOOGOL.                  ",
            "      *GOOGOL*--..:googol__                   ",
            "              -GOOGOLGOOGOL:telegram.         ",
            "              GOOGOL:           .+GOOGOL.     ",
            "              -GOOGOL+.      .+GOOGOL.        ",
            "                *GOOGOL=- .-=GOOGOL:          ",
            "                  +GOOGOLGOOGOL+              ",
            "                     `-googol-                ",
        ],
        colors: vec![Color::Green, Color::Black],
        width: 48,
    }
}

fn void_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "    _______    ",
            " _ \\______ -   ",
            "| \\  ___  \\ |  ",
            "| | /   \\ | |  ",
            "| | \\___/ | |  ",
            "| \\______ \\_|  ",
            " -_______\\     ",
        ],
        colors: vec![Color::Green],
        width: 15,
    }
}

fn nixos() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "          ::::.    ':::::     ::::'         ",
            "          ':::::    ':::::.  ::::'          ",
            "            :::::     '::::.:::::           ",
            "      .......:::::..... ::::::::            ",
            "     ::::::::::::::::::. ::::::    ::::.    ",
            "    ::::::::::::::::::::: :::::.  .::::'    ",
            "           .....googol:::googol: .googol    ",
            "          .googol::::.googol;;. googol      ",
            "         .googol:::::googol 'googol'        ",
            "     .....googol:::::googol::::..           ",
            "    :::::::::::googol:::::::::::'           ",
            "   '::::::::::.googol::::::::::             ",
            "        .....  `googol:::::::'              ",
            "       .googol.  ::::.googol                ",
            "      .::::::::  ::::::::.                  ",
            "     ':::::::::' .::::::'                   ",
            "             ...googol                      ",
        ],
        colors: vec![Color::Blue, Color::Cyan],
        width: 45,
    }
}

fn nixos_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "  \\\\  \\\\ //  ",
            " ==\\\\__\\\\/ // ",
            "   //   \\\\//  ",
            "==//     //== ",
            " //\\\\___//    ",
            "// /\\\\  \\\\==  ",
            "  // \\\\  \\\\   ",
        ],
        colors: vec![Color::Blue],
        width: 14,
    }
}

fn alpine() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "       .hddddddddddddddddddddddh.        ",
            "      :dddddddddddddddddddddddddd:       ",
            "     /dddddddddddddddddddddddddddd/      ",
            "    +dddddddddddddddddddddddddddddd+     ",
            "  `sdddddddddddddddddddddddddddddddds`   ",
            " `ydddddddddddd++hdddddddddddddddddddy`  ",
            ".hddddddddddd+`  `+ddddh:-sdddddddddddh. ",
            "hdddddddddd+`      `+y:    `+dddddddddddh",
            "ddddddddh+`   `//`   `.      -sddddddddddd",
            "ddddddh+`   `/hddh/`           `-sddddddddd",
            "ddddd:`    `/+/dddddh/`           `-/telegramdd",
            "ddddh`    `/`  `hddddddh/`           `telegrame",
            "ddddh`  `//`    `hddddddddh/`           `d",
            "ddddh` `++------+hddddddddddh/`         d",
            "ddddh./+++++++++++hddddddddddddh.      d",
            "dddddh+++++++++++++ddddddddddddddh    d",
            " hddddh+++++++++++++dddddddddddddh   d ",
            "  hddddh++++++++++++ddddddddddddh  d   ",
            "   ydddddh+++++++++++ddddddddddy     ",
            "    .hdddddh++++++++++dddddddh.      ",
            "       '+hddddh++++++++hddh+'        ",
        ],
        colors: vec![Color::Blue],
        width: 42,
    }
}

fn manjaro() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "██████████████████  ████████   ",
            "██████████████████  ████████   ",
            "██████████████████  ████████   ",
            "██████████████████  ████████   ",
            "████████            ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
            "████████  ████████  ████████   ",
        ],
        colors: vec![Color::Green],
        width: 31,
    }
}

fn manjaro_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "||||||||| ||||",
            "||||||||| ||||",
            "||||      ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
            "|||| |||| ||||",
        ],
        colors: vec![Color::Green],
        width: 14,
    }
}

fn endeavouros() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                     ./:               ",
            "                   ./+++:              ",
            "                 .:+++++/.             ",
            "                /+++++++++:            ",
            "              :++++++++++++/           ",
            "            `/+++++++++++++/.          ",
            "           ./++++++++++++++/.          ",
            "          ./++++++++++++++++/`         ",
            "         -+++++++++++++++++//.         ",
            "        :+++++++++++++++++++//-        ",
            "       /+++++++++++++++++++//+/.       ",
            "      /++++++++++++++++++++/::+:.      ",
            "    .+++++++++++++++++++/:`   `-:.     ",
            "   .++++++++++++++++//-`        `      ",
            "   ++++++++++++++/::`                  ",
            "  :+++++++++++:-`                      ",
            " `++++++++/:`                          ",
            " /++++++:.                             ",
            ".+++++:`                               ",
            "-+++:`                                 ",
            ":/:`                                   ",
            ":`                                     ",
        ],
        colors: vec![Color::Magenta, Color::Red, Color::Blue],
        width: 40,
    }
}

fn pop_os() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "             /////////////             ",
            "         /////////////////////         ",
            "      ///////*767telegramtelegram      ",
            "    //////telegramtelegram/////////    ",
            "   /////telegram telegram//////////   ",
            "  /////telegram   //telegram///////  ",
            " /////telegram///telegram//////////  ",
            "////////telegram telegram//////////  ",
            "////////telegram/telegram//////////  ",
            "////////telegram telegram//////////  ",
            " ///////telegramtelegramtelegram///  ",
            "  //////telegram telegram//////////  ",
            "   /////telegramtelegram//////////   ",
            "    //////telegramtelegram///////    ",
            "      ///////*767telegramtelegram    ",
            "         /////////////////////       ",
            "             /////////////           ",
        ],
        colors: vec![Color::Cyan, Color::White],
        width: 40,
    }
}

fn mint() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "             ...-:::::-...              ",
            "          .-MMMMMMMMMMMMMMM-.           ",
            "       .-MMMM`.telegram.-'MMMM-.       ",
            "     .-MMMM`  .MMMMMMMM.   `MMMM-.     ",
            "   .MMMM'  .MMMMMMMMMMMMM.   'MMMM.    ",
            "  .MMMM'  .MMMMMMMMMMMMMMM.   'MMMM.   ",
            " MMMM'  .MMMMMMMMMMMMMMMMMM.    'MMMM  ",
            "MMMM'  .MMMMMMMMMMMMMMMMMMMM.    'MMMM ",
            "MMMM'  .MMMM'      'MMMM'          MMMM",
            "MMMM'  .MMMM'       MMMM'          MMMM",
            "MMMM'  .MMMM'       MMMM'          MMMM",
            "MMMM'  .MMMM'       MMMM'          MMMM",
            "MMMM.  .MMMM        MMMM.        .MMMM ",
            " MMMM.  MMMM        MMMM.       .MMMM  ",
            "  MMMM. MMMM        MMMM.      .MMMM   ",
            "   'MMMM.MMMM      MMMM.     .MMMM'    ",
            "     'MMMM.MMMM   MMMM.   .MMMM'       ",
            "        'MMMMMMMMMMM    .MMMM'         ",
            "          `MMMMMMMM' .MMMM'            ",
            "             ''MMMM.'                  ",
        ],
        colors: vec![Color::Green, Color::White],
        width: 42,
    }
}

fn elementary() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "         eeeeeeeeeeeeeeeee            ",
            "      eeeeeeeeeeeeeeeeeeeeeee         ",
            "    eeeee  eeeeeeeeeeee   eeeee       ",
            "  eeee   eeeee       eee     eeee     ",
            " eeee   eeee          eee     eeee    ",
            "eee    eee            eee       eee   ",
            "eee   eee            eee        eee   ",
            "ee    eee           eeee         ee   ",
            "ee    eee         eeeee          ee   ",
            "ee    eee       eeeee            ee   ",
            "eee   eeee   eeeeee             eee   ",
            "eee    eeeeeeeeee              eee    ",
            " eeee    eeeeee              eeee     ",
            "  eeee                     eeee       ",
            "    eeeee               eeeee         ",
            "      eeeeeeeeeeeeeeeeeeeee           ",
            "         eeeeeeeeeeeeeee              ",
        ],
        colors: vec![Color::Blue],
        width: 38,
    }
}

fn zorin() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "        `osssssssssssssssssssso`        ",
            "       .osssssssssssssssssssssso.       ",
            "      .+oooooooooooooooooooooooo+.      ",
            "                                        ",
            "``````````````````````````````````````  ",
            ":::::::::::::::::::::::::::::::::::::: ",
            "``````````````````````````````````````  ",
            "                                        ",
            "      .+oooooooooooooooooooooooo+.      ",
            "       .osssssssssssssssssssssso.       ",
            "        `osssssssssssssssssssso`        ",
            "                                        ",
            "                                        ",
            "``````````````````````````````````````  ",
            ":::::::::::::::::::::::::::::::::::::: ",
            "``````````````````````````````````````  ",
            "                                        ",
        ],
        colors: vec![Color::Blue, Color::Cyan],
        width: 42,
    }
}

fn kali() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "      ,..,                                    ",
            " ,'     `,                                    ",
            ",'  ___    _ __ _ __    `.                    ",
            " / __ \\| '_ `| '_ \\      `                   ",
            " | |__) |  __/ | | | :                        ",
            " |____/\\___|_| |_|   ;                        ",
            " ,.                    ;                      ",
            ",        telegram  ,_,'                        ",
            " ,telegram telegram `,                         ",
            "   , telegram telegram `,                      ",
            "    `,       telegram,'                        ",
            "      `-.,googol-'                            ",
        ],
        colors: vec![Color::Blue, Color::Black],
        width: 46,
    }
}

fn parrot() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "  `:oho/-`                              ",
            "`yyyyyyyyyyyo/`                         ",
            "`yyyyyyyyyyyyyy/`                       ",
            " yyyyyy  `-yyyyyy/`                     ",
            " yyyyyyy      -yyyy+.                   ",
            " yyyyyyyy        +yyy+`                 ",
            " yyyyyyyyy`        .oyy+`               ",
            " `yyyyyyyyy`          +yy+              ",
            "   yyyyyyyyyy`          oyy.            ",
            "    `yyyyyyyyyy.          +ys           ",
            "      `yyyyyyyyys`          sy.         ",
            "         yyyyyyyyy+          +s/        ",
            "           yyyyyyyy+`         -s:       ",
            "            .yyyyyyyy:          +:      ",
            "              `:yyyyyys`         :.     ",
            "                  .+yyyyo.        .     ",
            "                      `:oyo:`           ",
            "                           ..           ",
        ],
        colors: vec![Color::Cyan, Color::Red],
        width: 42,
    }
}

fn slackware() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "                  :::::::                  ",
            "             :::::::::::::::               ",
            "          :::::::::::::::::::::            ",
            "        :::::::cllc:::::::::::::           ",
            "     :::::::::lool:::::::lllll:::          ",
            "    :::::::::lool:::::::loooooo:           ",
            "  :::::::::::lool:::::::looooooo:          ",
            " :::::::::::::lool::::::loooooooool        ",
            ":::::::::::::::lool:::::looooooooool       ",
            "::::::ccc::::::lool:::::looooooooool       ",
            ":::::loooc::::::lool::::loooooooool        ",
            "::::::loooc:::::::::::::loooooool          ",
            " ::::::loooccc::::::::looooooo:            ",
            "  :::::::loooooollllllllll:                ",
            "    ::::::clooooooooooll:                  ",
            "      ::::::llloooollll:                   ",
            "        :::::::cccc:::                     ",
            "           :::::::                         ",
        ],
        colors: vec![Color::Blue, Color::White],
        width: 44,
    }
}

fn linux() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "        #####           ",
            "       #######          ",
            "       ##O#O##          ",
            "       #######          ",
            "     ###########        ",
            "    #############       ",
            "   ###############      ",
            "   ################     ",
            "  #################     ",
            "#####################   ",
            "#####################   ",
            "  #################     ",
        ],
        colors: vec![Color::White, Color::Yellow],
        width: 24,
    }
}

fn linux_small() -> AsciiArt {
    AsciiArt {
        lines: vec![
            "    ___    ",
            "   (.. |   ",
            "   (<> |   ",
            "  / __  \\  ",
            " ( /  \\ /| ",
            "_/\\ __)/_) ",
            "\\/____\\/   ",
        ],
        colors: vec![Color::White],
        width: 11,
    }
}
