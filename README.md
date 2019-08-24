# iron
A small, and unsophisticated, C compiler - written in Rust!

Referenced from https://norasandler.com/2017/11/29/Write-a-Compiler.html

## Lexing
The lexer (also called the scanner or tokenizer) is the phase of the compiler that breaks up a string (the source code) into a list of tokens. A token is the smallest unit the parser can understand - if a program is like a paragraph, tokens are like individual words. (Many tokens are individual words, separated by whitespace.) Variable names, keywords, and constants, and punctuation like braces are all examples of tokens. Here’s a list of all the tokens in return_2.c:

int keyword
Identifier “main”
Open parentheses
Close parentheses
Open brace
return keyword
Constant “2”
Semicolon
Close brace
Note that some tokens have a value (e.g. the constant token has value “2”) and some don’t (like parentheses and braces). Also note that there are no whitespace tokens. (In some languages, like Python, whitespace is significant and you do need tokens to represent it.)

Here are all the tokens your lexer needs to recognize, and the regular expression defining each of them:

Open brace {
Close brace }
Open parenthesis \(
Close parenthesis \)
Semicolon ;
Int keyword int
Return keyword return
Identifier [a-zA-Z]\w*
Integer literal [0-9]+
If you want, you could just have a “keyword” token type, instead of a different token type for each keyword.

☑ Task:
Write a lex function that accepts a file and returns a list of tokens. It should work for all stage 1 examples in the test suite, including the invalid ones. (The invalid examples should raise errors in the parser, not the lexer.) To keep things simple, we only lex decimal integers. If you like, you can extend your lexer to handle octal and hex integers too.

You might notice that we can’t lex negative integers. That’s not an accident - C doesn’t have negative integer constants. It just has a negation operator, which can be applied to positive integers. We’ll add negation in the next post.
