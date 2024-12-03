use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::str;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Mul(i32, i32),
    Do,
    Dont,
}

fn main() -> Result<()> {
    let input = "-select()&how()''from()}what()mul(667,142);*when()*/%%+ &mul(139,116),,)mul(665,813)$>-+from()where(),from()mul(589,293))mul(832,177)mul(701,929)~([mul(300,986)from()mul(238,716)/~*~'what()when():}mul(437,789)mul(662,564)*)^,;%<}#'mul(567,346)don't()from()-who(),^/mul(939,542)mul(944,786)^}select()(mul(508,210)}mul(873,534)}select()where()-,*+)#@mul(925,280)where()*who()why()who()?%:mul(858,972)+mul(113,350)' *who()+from(641,444))&]*don't();from()+,@[mul(522,403)mul(627,14)why()where()+/#^!what()^mul(678,234) <mul(520,505):how()()[from()select()mul(500,879)~{:-:{~mul(163,615)[~;>mul(79,303)from()from()^'who()@;/~[mul(752,220);-don't()((select()select(){mul(454,158) *]#mul(387,262)how()where()<select()#]mul(904,366)]?<what()>when()[from(118,182)mul(662,176)%]')from()where()mul(259,178)--)~>$who()do()where()&&]~@{)%]mul(566,269);select()%}%+ mul(21,147),#&)*'why()#+don't()-,:{/%what()$*mul(883,511)select(264,142)select()how()]/?&&mul(205,770)select()%from()what()&$select()who())mul(538,114)/mul(9,272)from()?(why(727,269)when()don't()>+/ when(),)mul(750,773)*['do()]mul(316,235)+{{mul(744,197)*#who()*}how()how()why())mul(932,794)*where())+<-/ mul(371,760)why() who()(select()@mul(595,45)who()%how()'when()>how(){why()[mul(123,779)!@where()(mul(988,620)~'*,?/#when()mul(612,810)mul(720,939)]/(,^&how()%^]mul(391,308);&@-mul(525,708)+what(352,33)when()from()why()} ~mul(79,405)@*where()! mul(865,388) <mul(33,948)~?(}!??+from()why()mul(302,201)[{mul(585,486)mul(821,363)who(314,751)what()*,]{when()mul(101,727)]}#)mul(808,363):?&#why()select()+mul(98,877) select()~<what(651,799)+;~ )mul(766,538):[<:#[mul(501,413);!why()<&>?[$what()mul(747,941)#[}from() ((mul(418,822)mul(90,638)select()(#;;<[+@mul(657,611)]}@)mul(30,874)@mul(525,145))who() where()+:~+]mul(176,137)what()] mul(381,84)):how()<who()from():&when()+mul(175,258);who(818,691)mul(634,219){select(846,719)'where()what()^;)[#mul(648,322) $mul(127#<{#mul(965,174)mul(611,623)#?%;,:,don't()* when()}&&mul(952,606),>mul(315,467)%($'(*]:%mul(792,258)]>'mul(534+[#>&)+why()mul(881,357):^*?what()who(514,882)where())}-mul(88,883[;^-#mul(208,713)}?;]!%mul(651,918);{),:what()*mul(528,154)):@*]; &%[mul(542,543)mul(480,848)why()mul(843,903)[select())-*@ ;who()mul(77,706)<[;}mul(303,968));why()mul(916,126)&&%/&mul(462,652)[!~()what()from()from()select()how()mul(730,810/,when(286,584)don't()^select()mul(609,219)(-~&who():mul(135,454)from()-%~]#mul(909,564)when()&why(){/~+?who()[do()mul(852,307)-[?;)how()do()[[*->{!#^mul(678,864)>when()%>!']$mul(16,905),!mul(111,596)why()$mul(2,397)/-@}~;where(942,895)do()who()where()<}when()?mul(846,421),?(where()why(),how()(&mul(643,890)%~*[;% $what()don't()^$&(##mul(112,802)where()^ &(&&{&do()+(!(mul(732,198)[}!'>^%mul(919,810)mul(342,316))}what()&~:~$<when()do()*what()+:>mul(822,157)-,{select()who()mul(907,36>where()/$!^<&$where()mul(10,661)-who()mul(496,782)@%%?'{[;(mul(110,115)~mul(447,69)*[what()[{]>-'@mul(197,810)select()what()who()mul(492,647)where()*[{ [~&what()$mul(24%<>mul(234,703)
)$~where()when()@when(),mul(939,238){from()(#!><mul(340,605)-~mul(211,57)#!where()#~+mul(21,844#where()when()who()#!why()^?{-mul(565,649)*how()mul(489,506)what()>-mul(44,294)what()where(244,294)*when()&&what()who()what()mul(173,998)+mul(915,924)mul(126{'*mul(849,404)#select()mul(480,290)('%what()}),%'mul(525,37) /;mul(371,488{<who(210,427)++select()who()@%mul(886,752)mul(251when()&']select()why(): ?mul(271,611)#what()from(667,868)*why()'$mul(979,932)where()mul(97,326)select()!#why()when()mul(408,436)<#[,when()$mul(962,805)})$'mul)&?><$,what()mul(652,384)+/+']:^;;what()mul(463,163)[$<who()mul(655,577)!&~why()-?mul(33,901)#what()!&why()>mul(692,303)@(&'+<mul(705,862)how()what(){>'how()mul(574,785)select()?why():<when()mul(828,721)'who()@,&#+&why()mul(319,336):*+),-mul(590,602)@;:}from()mul(392,532),do()how(97,299);'when()mul(443,364)/<what()@why()how(267,551)where()&$;mul(939,815)[#mul(247,538)what()how()?mul(481,220)mul(761,577):who()from() ^}&+mul(331,667)select() >>when(579,794)>mul(385,563)>-%where():(^)mul(349,597)select()- '(why())mul(325,80)mul(83]}[[^*how() why()<mul(985,98)how()who())mul;<:mul(644,216)'who()$<^!select()@,mul(287,310)%select()^what(){<(*,mul(589,290)^-/;#select()mul(244,439)mul(28,263),how()$;,)!$<mul(676<mul(439,221)%[;mul(192,22)[mul(304,482),-mul(757,972)mul(489&what()~&^:'~<mul(483,773)+@}> don't()mul(807,644)-who()$mul(716,166);select()mul(626,79)/don't()!( why()mul(496,577)~how()':!what()'@,mul(44what()where()how()?who()<mul(21,315)from()#when():when()mul(859,48)$when()-,+mul(888,917)mul(609,468)(]}$where()~mul(936,753),how()+how()>mul(115,627)^&'from()% mul(28,698)+why()mul(7,895)'mul(150#$[{<!#>]:mul(370,234)}why()# &{)mul(902,182))}*+^where():why()mul(263,783)who()]#;(-!why()$mul(635,217from()from();%+:$;^)mul(505,91)}''%${mul(467,336)}]]mul(323,115)^^why()why()%?>$>!mul(314,474)when()!'},%*select(),when()do()]':mul(748,29)why()select()'&^}~&from(),mul(55,11)[*/&who()&&mul(478,127)]-when()mul(64,831)what()/;[[mul(822,263)mul(833,138)>,$;how(843,918)>#mul(180,9)&what(136,915):;mul(883,785)mul(544,853)%mul(547,842)?select()^~:((mul(475,655)'^+[mul(70,929)when()from()+from()}[why()?'why()mul(88,259)from()[:who()<<mul(201,880)/)&mul(824,474){?what()where()+what()when()!&,mul(405,309)mul(74,297)$[how()mul(640,143);*'?&>mul(140,477)>&$where(602,252)-;;% {mul/@ ${select() !mul(167,943)what()[)mul(753,926)}$+#$mul(167,13)mul(456,116)mul(109,352)>]select()/'<mul(648,515)select()why()don't()mul(132,771)[[ mul(160,607)from()mul(495,410){-!what(731,360)from()> when()!mul(820,444),, (*mul(227,538)@:%{mul(168,817)from()/where():$ />who(795,19)-mul(799,466)~;(;from()-$)){mul(356,431)!from()*$what()mul(175,543)*mul(340,188)when():where()mul(613,955)from()~do()where(){mul(733,695) @mul(96,566)how()^what()how()'$what()]mul(303,135)]//who()(who()+mul){;^,mul(515,424) ]/%select()]mul(283,274)'>;*~from(){{from()?mul(869,316)
';[why()mul(719,314)-mul(38,357):]how()*/how()select()mul(78,942);$+#from()mul(265,330)from()from()& +-@-,mul(148,251)where()})why()/mul(159,418)%<},-?!^!~mul(632,32),don't()what()what()]>when()-#when()]mul(518,898)mul(64,74)mul(624,359){-*}mul(435,903)how()select()%where(36,5)(mul(750,885)*where()mul(15,112)<why() (who()select()}!mul(301,774)*&}/*~select(),mulwhere()who()do()from()#{#[&)[mul(485,891)mul(645,778)-#>?:%select()@<[mul(913,903));[from()#&*mul(511,123)+%when()&^'#mul(495,296,;what()mul(56,831) #[>/*(>+^mul(806,137)))/@*@}mul(395,547)'select(){what()why()*where():!don't()~!mul(112,799)(%from()*who(172,318)mul,)!&mul(590,625)$mul(516,115)how()[*!>mul(914,404)~when()](what()*mul(400,936)how()*^^{<{%?mul(999,987)!})#]mul(959,690)@{'}:{when()when()mul(570,284);~mul(97,397)@;#mul(132,335)?~#mul(42,819))mul(655,746){#^%who(101,15)&#!mul(291,803),#+?&mul(816,961);}mul(122,643)?~*#what()@;what()mul(711,201)why()(~]?&mul(529,206)from()where()%@from() /]mul(444,963)@who()from()/:[(-mul(346,877))/<!;&why()mul(883,670)[&how()+<why()select()!mul(257,88)from()-->when()&mul(177,443)$![%<mul(138,786)!%select()%what()-mul(71,360) ,!mul(852select(981,300)@mul(582,394)^/$,why())~%mul(511,491)[,)$~[{$^!mul(904>;@don't()][mul(677,276)*mul(353,913)select()from()*mul(695,847)/mul(20,815()who())})mul(438,161)select():why()who()*]^]mul(321,822)*^from()/where()-?{mul(718,349)mul(355,582)<;mul(830,962)mul(312,508))from()where()who())&why()>]do()mul(224,71)@(how()why()select(688,447)mul(89,792)#mul(154,938)select()do() who()}$!!&mul(172,622))$what(){>mul(623,570)&%-where():&*}mul(31,554)<(#where()*from():{@!mul(987,776))*'/mul+?(mul(701,61)~];}><{&mul(563,85):^^how()(mul(868,860)~,what()@{do()'why()mul(819,142)<$how()&mul(299,411)?mul(954,291){'-where()>mul(985 ~^why()?mul(131,20)*/ >from()',++>mul(415,335)?[-mul(798,190)^who()>;[mul(611,479)mul(658,622~,who()/&#:{who()}?mul(291,148)+where()}@who(51,417)what()why())<mul(568,202)>why()?:{*[!;mul(627,512):[;<':*mulwhy()((@)$$ ':don't()what(511,960)mul(159,248); mul(588,419)-#%<when()mul(372,2)'!+<-mul(372,361)+{why()(how()%do()]who()mul(933,797)%}>(mul(331,93}from()>mul(560,438)}from()mul(729,489)}*<@what()mul(456,993)]+-('@mul(164,410)@%+mul(283,493)<*;/ :when()((mul(266,486)[when(),?+mul(249,165)}don't()+why()where()'~mul(472,202),select(554,790)mul(387,509)~&!mul(514,914)who()from()when()mul(673,224)@where()/</mul(113,935)@;mul(900,18)~(mul(246,115)mul$!-^%'@!#)@mul(39,411)$^-mul*how() {$mul(791,482)what()/ when()don't()#why(168,204) how()$(!&mul(293,987)$?where(934,585)what()'mul(118,985)>++)+{ +mul(480,948)$/when()don't()&,mul(584,391) @+mul(180,685)who()select()'-!!$~mul+?mul(249,664)why()select()<mul(200,510)#(}select()mul(902,883)+when()mul(552,250))(what()select()(^(where()when()mul(310,298)who()how()/}select()#!/usr/bin/perl)<)how(),)how()'{@mul(438,954)mul(658,687)
:<mul^'}how()mul(611,205)where(86,446){when()how()-from()mul(662,949):select(){{[[from()'!mul(275,341)';-;~mul(889,976)@:^#who(43,814)mul(105,170)) ::*why(88,387)[ mul(136,219)+> mul(627,626)'$who()how()~mul%)@+;[{mul(237,229)who()%<{-{*]+;mul(809,56)#~}'mul(691,801)(:}['{<-/mul(990,19)select()where()what()':who(194,310)!^{mul(972,914)];&>mul(823,879)mul(531,912)how(640,856)^who()%!@mul(988,624)why() ^~[mul(593,75)what()*}]#-mul(238,652)$why(){#:$,)mul(351,278)@select() @[*/'mul(41,175)mul(723,126)&+how()/when()%mul(495,494what(25,590)why():]*$(mul(760,407) mul(208,295)*when()&'}do()@ }&when()/~who()what()^mul(613,494) why()%~@who()mul(159,575)why()where(390,372)%% ~from()-mul(570,730))mul(634,554)who()&how()mul(909,159)[[)+mul(481,13) mul(310,211)what()}why()from()how()< $why()mul(235,144):${how()^?;!]mul(601,971)mul(3,148)>))mul(351,281)from()&why(){!from()mul(398,4)why()when()#>mul(779,170)+)when()@>]/]mul(374,400)mul(725,109)from() *-<[+'mul(771,683)where()how()%+(,(mul(968,53)['who()@>[@(select():mul(650,309)#*,mul(676/#how()@select()^}%<where();mul(725,606),#mul(492,472)@>>from())mul(428,451)%}where()({%{>what()how()mul(840,189)how()mul(734,158)[:#what()&&(when() -mul(302,951){-what()>%[+mul(86,676)!mul(574,410)/*who()mul(709,782)mul:how()[)%-&from()mul(903,410)&~^!why()select()>when()how()mul(501,677)&,where()$)>'mul(572,739){mul(26,438)(*where()^'&mul(437,403)where()what()mul@&[<*)[:#%%mul(523,163)mul(401,39%&/]>mul(712,673)mul(593,248) mul(910,410)+?mul(60,141)-who()?~'mul(798,372)mul(469,939)(select()^[!-'mul(243,488)&/+-/^what()>^>mul(618,943 ('mul(252,224)*!$mul(619,792)from()&^mul(218,278){,mul(416,669)+*$]#mul(800,119)%]%^#$;*>{mul(482,893)'&&?how()mul(89,677)&)#<from(826,136)-+where()/>mul(483,457)why()[select()(!+;[mul(134,789)]~how() #%mul(837,402)~,?>*mul(128,824)<select()&#+ mul(306,537)where()mul(263,130)/<,~@where()who();?mul(297,684): mul(366,263)!;'^mul(957,894)mul(725,81)~ /{ ~mul(310,669)#mul(674,149)[mul(237,259){+)$^<+%{mul(237,745)when()}[what(779,966)>[/#@don't()what()$[&mul(181,943)[mul(678,31)~$:who()when()@ *what()mul(141,981)>!!~,/mul(169,72)where()why(),+*mul(347,11)~what(),when(): when()]mul(283,646)?'mul(834,247)[mul(744,766)()*mul(979,782)$;mul(864,806)?from()mul(836,818){%mul(662,438)mul(174,824)<mul(637,437)from()-}[?>'who()+select()mul(614,265)^select()mul(141,687)+**)[~$[/-mul(61,197)from() when()mul(498,859)who()@>why()how()how()mul(494<mul(357,897)mul(947,905)+~<^mul(672,858)how()how()&}%mul(340,814)}^^<mul(701,475)>when()!^^don't()when()mul(29,653){why()mul(41,526)#select()from()mul(715,903)
@where(957,13)how(451,967):?(/mul(884,339)&: -mul(895,988)%:from()mul(267,780)do()+!mul(747,812)[*;:&-##[,mul(877,201)@[from()(<{)<$mul(696,261)where()/??&-$:where()mul(451,383)#how()}$>,mul(966,386)~-#from()!when()/what()select()mul(893,103)/ ; /;-select(467,60)why(139,346)?mul(650,959)why(884,941)+why()<mul(848,148)how()what()(}who()*}>mul(353,814)from()who(){-*#who()mul(400,173)mul(661^$[[:*?mul(731,905)&(mul(317,300)!^< [/-how(){when()mul(476,578)who()from()&}-)mul(628,306)who():~',+@mul(633,755)$?^&mul(45,740)mul(495,369)how()when()!select()mul(809,627){when() what()mul(625,205)%/select()$(where():why()^when()don't()<where()?where()< [#mul(36,868))when())~mul@-<:@what();mul(68,63)+[%@@how()%~why()mul(831,988)!who(160,84)how()<-;{mul(927,989)select()) who() mul(994,339?^>>/:?mul(825,853):when()&~,(~select()]<mul(935,352)#} $^where(690,579)where(213,186):don't())why() +,,mul(666,304)}}&when()}mul(637,787)how()-why()>~mul(660,614)]+mul(968,860)-do()%select()~ mul(843,731!$}how()mul(354,619)@select()mul(354,560)when()}*mul(994,176)~]#how(549,822)[mul(887,251)[select()&*%who(17,670)who()%)&mul(360,653)-/''>from(803,726)mulwhere()?what()&mul(446,573):select()?how():&mul(660,62);[;)/?!@ when()mul(131,738)~;)mul(222,467)how()@},who()>why()mul(907,236)-$]^mul(389,882){mul(481,917)how()--!{when()mul(826,862)!what(776,585)%*from()/^mul(258,741)&+@[,&from()^?>mul(615,874))#where()who()^from()'(what()mul]#mul(239,843)where()where()!&mul(35,752)#}}how()))mul(103,32)'>]mul(491,16)%$'mul(914,334)~select()/'!@mul(256,451) ~%[$why()?mul(330,802)where(){(?mul(373,28[#:&/&'%%mul(227,364))mul]:mul(650,371)@@?[$from()mul(358,277)mul(102,94)%)';mul(431,297)>!when()*mul(83,650)#;/'@<)!/mul(444,600)+(*mul(342,980)$;mul(887,991)@mul(942,266)mul(554,806)[{why()who(), )select(){mul(560,307)^)mul(90,988)*:[mul(362}&don't()]where()what()!?!$mul(646,5)[+&(;;/^mul(70?'mul(768,397)]where()#(%don't()-how()~select()^how()@mul(969,59)<[how(288,620)who()>what()?from(925,90)mul(644,250)@+)@how()from()?from()mul(370,958)}mul(856,740)don't()](<#->!who()*mul(127,512)where(155,111)&$**>?mul(568,551)(when();$} (why()#mul(696,66)]~{/]mul(54,72)}mul(723,615)#mul(746,486)['*mul(890,144)!mul(139,363)mul(203,759),<;mul(954,65)when()*mul(122,219)^what()who(31,222)how()when()how()mul(235,569)what(934,341)from(545,730)/who()*^mul(344,502)where()~-] $>$mul(473,194)mul(514,408)do()when()['mul(592,732)mul(668,655){what()]>+?mul(897,859)~$select()]mulwhere()mul(116,140)@+what()&#mul(843,710)/$%~; @mul(329,45)~?<who():'@?/mul(178,435)+;what()/mul(224when(),do()!$mul(29,457)::;;;do()(when()>?@mul(751,862)?>mul(465,182)mul(879,670)select()}<)how()select(342,147)^why()-why(77,328)mul(839,63){from()mul(609,495)who())??#@%&}mul(791,585)mul(89,505)who() ~who()  $do()$;][mul(208,816)$;mul(162,345)?)what()*#^$]what()mul(239,645)from(),{@mul(225,844)select()'why(609,236)}?[mul(364,169)(select()!mul(273,586)
from()where()'%'where()<{?}mul(758,234)from(443,726)select()when()}'mul(884,68)**/[&{;~mul(818,66)why()where()>,from()-?;{mul(282,741)?['>mul(560,385)why()]@&$]<>]from()do()}/*^!}{mul(142,84)$from()who()how()mul:mul(225,76),how() (#how()mul(685,905)when()!!^['(^mul(744,487)-mul(837,928)from()<$((mul(776,854)mul(668,869)!who()mul(123,137)%-mul(76,346)-where()select()^}mul(912,946)'$#!mul(930,449)(,where(),$-how(){do()&what(),]when(){mul(665,336)$don't()<%why()]-}why()how()mul(705,882)select()+%-&#mul(162,959)select(),/*^^why()-+?mul(665,535)~?what()(select()how()from()how()^what()mul(968,599){,~##how()#-,(mul(620,224)^{+ mul(864,140)where()mul(169,847):why()?what()+don't()mul(251,569)/#%>mul(956,70);:mul(200,261)>!@when()):<^+:mul(899,267)who()#,&from()what()when(473,423)'from()mul(521,38)(why()/why()>^/]?mul(881,241)#'*(/mul(991,447)*mul(481,364)!what()$+^%mul{-what()}:+;mul(260,175),}$+what()mul(476,265)who()how();%#mul(444,418)>)select()from()why()'mul(189,265)@[({}mul(736,742)when()],mul(689,493)+ ^#[:don't()<mul(855,735[!-why()~?*mul(229,961)<:mul(706,567)@when(923,459)?)from()mul(540,764)-@select()!;)how(882,29)@~from()mul(641,652) why(396,613)?}when()from()when(){mul(395,627)>where()select()#mul(524,647)where()'%#!}(mul(466,834)))%where()~>!]mul(841,130)%select()>how()<select()mul(127,544)'{>why();mul(511,221)when()<!]mul(583,651)%mul(278,443)'-/mul(964,156);don't()/who()]#;$<@mul(787,906)>-mul(295,571)<*};why()?[~mul(749,813) ~*(why()^why() mul(74,836)%:%(/>/%)mul(148,705)&mul(973,710)]!}:]from()!who()-mul(751,18)/#[from()>/'what()/mul(451,419)how()^where(),&>'mul(21,960){+-{<mul(842,588)/}mul(505,70)#who()}%{how()@(%?mul(166,516)'!<who()when()}/-!mul(204,793)>'mul(305,674)%]&*- {-],mul(241,623)!]who()how();%don't()mul(987,838)+mul(495,346)>#*[what()@[select()don't()??:^select(),)mul(83,151);;]:-'#from()when()who()mul(641,58)<$,where()!why()+mul(698,90)from()mul(289,417)^from()who()<+>mul(280,434)!mul(510,875)select()from()]mul(63,720)from()*{who(140,649)- how() -]mul(310,719)[;why()mul(199,500)>who()mul(150,452)why()^mul(573,483)mul(194,434)select()$]$++mul(793,362)}where():*when()&:who())mul(448,657)who()don't()?-:-[!'mul(557,248)! @/why()[<)when(245,258)%mul(874,939)what()^from())mul(55,802)^%,&?select(431,807)}from(331,380)~where()mul(714,95)$?,mul(574,272)mul(895,71)){how(),/]mul(600,135)mul(875,178)]}$^ ?what()?{]mul(55,317)(^)(mul(474,170)when())]!what()mul(872,189)'from()where()( [$<)mul(948,648)-^[&#mul(308,675)[what()]from():(mul(547,600)how()who()~[what()~+who()mul(107,328)~([:+mul(982,96)>:mul(100,141)*'&(~where()[mul(528,201)>what())/~^<what()mul(646,182)what()# (-mul(439,233)mul(647,936)~()mul(99,510)[&~>:;mul(10,602)";

    // Part 1
    let product = parse_all_mul(input).iter().map(|(a, b)| a * b).sum::<i32>();
    println!("Product: {}", product);

    // Part 2
    let commands = process_commands(input);
    let product = commands
        .iter()
        .map(|command| match command {
            Command::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum::<i32>();
    println!("Product (Part 2): {}", product);

    Ok(())
}

// Parse single mul command
fn parse_mul(input: &str) -> IResult<&str, Command> {
    map(
        delimited(
            tag("mul("),
            separated_pair(
                map_res(digit1, |s: &str| s.parse::<i32>()),
                char(','),
                map_res(digit1, |s: &str| s.parse::<i32>()),
            ),
            char(')'),
        ),
        |(a, b)| Command::Mul(a, b),
    )(input)
}

/// Parse a `do()` command.
fn parse_do(input: &str) -> IResult<&str, Command> {
    map(tag("do()"), |_| Command::Do)(input)
}

/// Parse a `don't()` command.
fn parse_dont(input: &str) -> IResult<&str, Command> {
    map(tag("don't()"), |_| Command::Dont)(input)
}

/// Extract and process all valid `mul` commands using `parse_mul`.
/// This is useful for part 1.
fn parse_all_mul(input: &str) -> Vec<(i32, i32)> {
    let mut remaining = input;
    let mut results = Vec::new();

    while !remaining.is_empty() {
        match parse_mul(remaining) {
            Ok((rest, Command::Mul(a, b))) => {
                results.push((a, b)); // Collect the values from the Mul command
                remaining = rest;
            }
            Ok((rest, _)) => {
                // Skip non-Mul commands
                remaining = rest;
            }
            Err(_) => {
                // Skip invalid characters
                remaining = &remaining[1..];
            }
        }
    }

    results
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    nom::branch::alt((parse_mul, parse_do, parse_dont))(input)
}

// Process all commands and filter out disabled `mul` commands.
fn process_commands(input: &str) -> Vec<Command> {
    let mut remaining = input;
    let mut results = Vec::new();
    let mut mul_enabled = true;

    while !remaining.is_empty() {
        if let Ok((rest, command)) = parse_command(remaining) {
            match command {
                Command::Mul(_, _) if mul_enabled => results.push(command),
                Command::Do => mul_enabled = true,
                Command::Dont => mul_enabled = false,
                _ => {} // Ignore disabled `mul` commands
            }
            remaining = rest;
        } else {
            // Skip invalid characters
            remaining = &remaining[1..];
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_part_1() {
        let a = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(parse_all_mul(a), vec![(2, 4), (5, 5), (11, 8), (8, 5)]);
    }

    #[test]
    fn parse_part_2() {
        let a = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            process_commands(a),
            vec![Command::Mul(2, 4), Command::Mul(8, 5),]
        );
    }
}
