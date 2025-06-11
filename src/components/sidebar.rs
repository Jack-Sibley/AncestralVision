use maud::{Markup, Render, html};

pub struct Sidebar {}
impl Render for Sidebar {
    fn render(&self) -> Markup {
        html! {
            aside {
	            nav {
			        p { bold { "1. Game Concepts"} }
			        ul {
				        li { a href="#" { "103. Starting the Game" }}
				        li { a href="#" { "104. Ending the Game" }}
				        li { a href="#" { "105. Colors" }}
				        li { a href="#" { "106. Mana" }}
				        li { a href="#" { "107. Numbers and Symbols" }}
				        li { a href="#" { "108. Cards" }}
				        li { a href="#" { "109. Objects" }}
				        li { a href="#" { "110. Permanents" }}
				        li { a href="#" { "111. Tokens" }}
				        li { a href="#" { "112. Spells" }}
				        li { a href="#" { "113. Abilities" }}
				        li { a href="#" { "114. Emblems" }}
				        li { a href="#" { "115. Targets" }}
				        li { a href="#" { "116. Special Actions" }}
				        li { a href="#" { "117. Timing and Priority" }}
				        li { a href="#" { "118. Costs" }}
				        li { a href="#" { "119. Life" }}
				        li { a href="#" { "120. Damage" }}
				        li { a href="#" { "121. Drawing a Card" }}
				        li { a href="#" { "122. Counters" }}
				        li { a href="#" { "123. Stickers" }}
			        }
	            }
            }
        }
    }
}
