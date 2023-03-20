package repl

import (
	"bufio"
	"fmt"
	"io"
	"os/user"
	"woc_lang/lexer"
	"woc_lang/parser"
)

const PROMPT = "∑(っ°Д°;)っ >> "

const LANG_LOGO = `

██╗    ██╗                         
██║    ██║                         
██║ █╗ ██║   ██████╗    ██████╗
██║███╗██║  ██║   ██║  ██╔════╝
╚███╔███╔╝  ╚██████╔╝  ╚██████╗
 ╚══╝╚══╝    ╚═════╝    ╚═════╝
`

const INTRODUCE_TEXT = `
嗨！嗨！嗨！%s 老板！你好呀，有幸在茫茫人海中遇到你，我是 WocLang 的开发者 o(^▽^)o，
这个语言只是我在业余时间自娱自乐写出来的，该语言目前并不完善，也没有什么牛逼哄哄的功能，
还配不上 Woc 这种程度的惊讶，但只要时间够，我会慢慢完善这个语言的，
最后感谢您愿意花上您宝贵的时间来瞥一眼这个编程语言，祝愿各位老板长命百岁，永远不死！！！诶嘿~~

欢迎试用 WocLang 语言！

`

func StartREPL(u *user.User, in io.Reader, out io.Writer) {
	scan := bufio.NewScanner(in)
	_, _ = fmt.Fprintf(out, LANG_LOGO)
	_, _ = fmt.Fprintf(out, INTRODUCE_TEXT, u.Username)

	for {
		_, _ = fmt.Fprintf(out, PROMPT)
		scanned := scan.Scan()
		if !scanned {
			return
		}

		line := scan.Text()
		if line == "exit" {
			_, _ = fmt.Fprintf(out, "%s老板再见！下次再来哦！！！", u.Username)
			return
		}

		l := lexer.New(line)
		p := parser.New(l)

		if len(p.Errors()) != 0 {
			printParserErrors(out, p.Errors())
			continue
		}

		_, _ = io.WriteString(out, p.Program.String())
		_, _ = io.WriteString(out, "\n")
	}
}

func printParserErrors(out io.Writer, errors []string) {
	_, _ = io.WriteString(out, "哦豁！好像有错误诶~~你瞅瞅？\n")
	for _, msg := range errors {
		_, _ = io.WriteString(out, "\t"+msg+"\n")
	}
}
