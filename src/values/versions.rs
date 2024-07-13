#![allow(non_camel_case_types)]

use crate::data_types::VarInt;

/// Enum of all versions of the game after the Netty rewrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// Snapshot 13w41b (0)
    Snapshot13W41B,
    /// Snapshot 13w42b (1)
    Snapshot13W42B,
    /// Snapshot 13w43a (2)
    Snapshot13W43A,
    /// Pre Release 1.7.1 (3)
    PreRelease1_7_1_PRE,
    /// Release 1.7.5 (4)
    Release1_7_5,
    /// Release 1.7.10 (5)
    Release1_7_10,
    /// Snapshot 14w03b (6)
    Snapshot14W03B,
    /// Snapshot 14w04a (7)
    Snapshot14W04A,
    /// Snapshot 14w04b (8)
    Snapshot14W04B,
    /// Snapshot 14w05b (9)
    Snapshot14W05B,
    /// Snapshot 14w06b (10)
    Snapshot14W06B,
    /// Snapshot 14w07a (11)
    Snapshot14W07A,
    /// Snapshot 14w08a (12)
    Snapshot14W08A,
    /// Snapshot 14w10c (13)
    Snapshot14W10C,
    /// Snapshot 14w11b (14)
    Snapshot14W11B,
    /// Snapshot 14w17a (15)
    Snapshot14W17A,
    /// Snapshot 14w18b (16)
    Snapshot14W18B,
    /// Snapshot 14w19a (17)
    Snapshot14W19A,
    /// Snapshot 14w20b (18)
    Snapshot14W20B,
    /// Snapshot 14w21a (19)
    Snapshot14W21A,
    /// Snapshot 14w21b (20)
    Snapshot14W21B,
    /// Snapshot 14w25a (21)
    Snapshot14W25A,
    /// Snapshot 14w25b (22)
    Snapshot14W25B,
    /// Snapshot 14w26a (23)
    Snapshot14W26A,
    /// Snapshot 14w26b (24)
    Snapshot14W26B,
    /// Snapshot 14w26c (25)
    Snapshot14W26C,
    /// Snapshot 14w27b (26)
    Snapshot14W27B,
    /// Snapshot 14w28a (27)
    Snapshot14W28A,
    /// Snapshot 14w28b (28)
    Snapshot14W28B,
    /// Snapshot 14w29a (29)
    Snapshot14W29A,
    /// Snapshot 14w30b (30)
    Snapshot14W30B,
    /// Snapshot 14w30c (31)
    Snapshot14W30C,
    /// Snapshot 14w31a (32)
    Snapshot14W31A,
    /// Snapshot 14w32a (33)
    Snapshot14W32A,
    /// Snapshot 14w32b (34)
    Snapshot14W32B,
    /// Snapshot 14w32c (35)
    Snapshot14W32C,
    /// Snapshot 14w32d (36)
    Snapshot14W32D,
    /// Snapshot 14w33a (37)
    Snapshot14W33A,
    /// Snapshot 14w33b (38)
    Snapshot14W33B,
    /// Snapshot 14w33c (39)
    Snapshot14W33C,
    /// Snapshot 14w34a (40)
    Snapshot14W34A,
    /// Snapshot 14w34b (41)
    Snapshot14W34B,
    /// Snapshot 14w34c (42)
    Snapshot14W34C,
    /// Snapshot 14w34d (43)
    Snapshot14W34D,
    /// Pre Release 1.8/1 (44)
    PreRelease1_8_PRE1,
    /// Pre Release 1.8/2 (45)
    PreRelease1_8_PRE2,
    /// Pre Release 1.8/3 (46)
    PreRelease1_8_PRE3,
    /// Release 1.8.9 (47)
    Release1_8_9,
    /// Snapshot 15w31a (49)
    Snapshot15W31A,
    /// Snapshot 15w31b (50)
    Snapshot15W31B,
    /// Snapshot 15w31c (51)
    Snapshot15W31C,
    /// Snapshot 15w32a (52)
    Snapshot15W32A,
    /// Snapshot 15w32b (53)
    Snapshot15W32B,
    /// Snapshot 15w32c (54)
    Snapshot15W32C,
    /// Snapshot 15w33a (55)
    Snapshot15W33A,
    /// Snapshot 15w33b (56)
    Snapshot15W33B,
    /// Snapshot 15w33c (57)
    Snapshot15W33C,
    /// Snapshot 15w34a (58)
    Snapshot15W34A,
    /// Snapshot 15w34b (59)
    Snapshot15W34B,
    /// Snapshot 15w34c (60)
    Snapshot15W34C,
    /// Snapshot 15w34d (61)
    Snapshot15W34D,
    /// Snapshot 15w35a (62)
    Snapshot15W35A,
    /// Snapshot 15w35b (63)
    Snapshot15W35B,
    /// Snapshot 15w35c (64)
    Snapshot15W35C,
    /// Snapshot 15w35d (65)
    Snapshot15W35D,
    /// Snapshot 15w35e (66)
    Snapshot15W35E,
    /// Snapshot 15w36a (67)
    Snapshot15W36A,
    /// Snapshot 15w36b (68)
    Snapshot15W36B,
    /// Snapshot 15w36c (69)
    Snapshot15W36C,
    /// Snapshot 15w36d (70)
    Snapshot15W36D,
    /// Snapshot 15w37a (71)
    Snapshot15W37A,
    /// Snapshot 15w38a (72)
    Snapshot15W38A,
    /// Snapshot 15w38b (73)
    Snapshot15W38B,
    /// Snapshot 15w39c (74)
    Snapshot15W39C,
    /// Snapshot 15w40a (75)
    Snapshot15W40A,
    /// Snapshot 15w40b (76)
    Snapshot15W40B,
    /// Snapshot 15w41a (77)
    Snapshot15W41A,
    /// Snapshot 15w41b (78)
    Snapshot15W41B,
    /// Snapshot 15w42a (79)
    Snapshot15W42A,
    /// Snapshot 15w43a (80)
    Snapshot15W43A,
    /// Snapshot 15w43b (81)
    Snapshot15W43B,
    /// Snapshot 15w43c (82)
    Snapshot15W43C,
    /// Snapshot 15w44a (83)
    Snapshot15W44A,
    /// Snapshot 15w44b (84)
    Snapshot15W44B,
    /// Snapshot 15w45a (85)
    Snapshot15W45A,
    /// Snapshot 15w46a (86)
    Snapshot15W46A,
    /// Snapshot 15w47a (87)
    Snapshot15W47A,
    /// Snapshot 15w47b (88)
    Snapshot15W47B,
    /// Snapshot 15w47c (89)
    Snapshot15W47C,
    /// Snapshot 15w49a (90)
    Snapshot15W49A,
    /// Snapshot 15w49b (91)
    Snapshot15W49B,
    /// Snapshot 15w50a (92)
    Snapshot15W50A,
    /// Snapshot 15w51a (93)
    Snapshot15W51A,
    /// Snapshot 15w51b (94)
    Snapshot15W51B,
    /// Snapshot 16w02a (95)
    Snapshot16W02A,
    /// Snapshot 16w03a (96)
    Snapshot16W03A,
    /// Snapshot 16w04a (97)
    Snapshot16W04A,
    /// Snapshot 16w05a (98)
    Snapshot16W05A,
    /// Snapshot 16w05b (99)
    Snapshot16W05B,
    /// Snapshot 16w06a (100)
    Snapshot16W06A,
    /// Snapshot 16w07a (101)
    Snapshot16W07A,
    /// Snapshot 16w07b (102)
    Snapshot16W07B,
    /// Pre Release 1.9/1 (103)
    PreRelease1_9_PRE1,
    /// Pre Release 1.9/2 (104)
    PreRelease1_9_PRE2,
    /// Pre Release 1.9/3 (105)
    PreRelease1_9_PRE3,
    /// Pre Release 1.9/4 (106)
    PreRelease1_9_PRE4,
    /// Pre Release 1.9.1/1 (107)
    PreRelease1_9_1_PRE1,
    /// Release 1.9.1 (108)
    Release1_9_1,
    /// Pre Release 1.9.3/1 (109)
    PreRelease1_9_3_PRE1,
    /// Release 1.9.4 (110)
    Release1_9_4,
    /// Snapshot 16w20a (201)
    Snapshot16W20A,
    /// Snapshot 16w21a (202)
    Snapshot16W21A,
    /// Snapshot 16w21b (203)
    Snapshot16W21B,
    /// Pre Release 1.10/1 (204)
    PreRelease1_10_PRE1,
    /// Pre Release 1.10/2 (205)
    PreRelease1_10_PRE2,
    /// Release 1.10.2 (210)
    Release1_10_2,
    /// Snapshot 16w32a (301)
    Snapshot16W32A,
    /// Snapshot 16w32b (302)
    Snapshot16W32B,
    /// Snapshot 16w33a (303)
    Snapshot16W33A,
    /// Snapshot 16w35a (304)
    Snapshot16W35A,
    /// Snapshot 16w36a (305)
    Snapshot16W36A,
    /// Snapshot 16w38a (306)
    Snapshot16W38A,
    /// Snapshot 16w39a (307)
    Snapshot16W39A,
    /// Snapshot 16w39b (308)
    Snapshot16W39B,
    /// Snapshot 16w39c (309)
    Snapshot16W39C,
    /// Snapshot 16w40a (310)
    Snapshot16W40A,
    /// Snapshot 16w41a (311)
    Snapshot16W41A,
    /// Snapshot 16w42a (312)
    Snapshot16W42A,
    /// Snapshot 16w44a (313)
    Snapshot16W44A,
    /// Pre Release 1.11/1 (314)
    PreRelease1_11_PRE1,
    /// Release 1.11 (315)
    Release1_11,
    /// Release 1.11.2 (316)
    Release1_11_2,
    /// Snapshot 17w06a (317)
    Snapshot17W06A,
    /// Snapshot 17w13a (318)
    Snapshot17W13A,
    /// Snapshot 17w13b (319)
    Snapshot17W13B,
    /// Snapshot 17w14a (320)
    Snapshot17W14A,
    /// Snapshot 17w15a (321)
    Snapshot17W15A,
    /// Snapshot 17w16a (322)
    Snapshot17W16A,
    /// Snapshot 17w16b (323)
    Snapshot17W16B,
    /// Snapshot 17w17a (324)
    Snapshot17W17A,
    /// Snapshot 17w17b (325)
    Snapshot17W17B,
    /// Snapshot 17w18a (326)
    Snapshot17W18A,
    /// Snapshot 17w18b (327)
    Snapshot17W18B,
    /// Pre Release 1.12/1 (328)
    PreRelease1_12_PRE1,
    /// Pre Release 1.12/2 (329)
    PreRelease1_12_PRE2,
    /// Pre Release 1.12/3 (330)
    PreRelease1_12_PRE3,
    /// Pre Release 1.12/4 (331)
    PreRelease1_12_PRE4,
    /// Pre Release 1.12/5 (332)
    PreRelease1_12_PRE5,
    /// Pre Release 1.12/6 (333)
    PreRelease1_12_PRE6,
    /// Pre Release 1.12/7 (334)
    PreRelease1_12_PRE7,
    /// Release 1.12 (335)
    Release1_12,
    /// Snapshot 17w31a (336)
    Snapshot17W31A,
    /// Pre Release 1.12.1/1 (337)
    PreRelease1_12_1_PRE1,
    /// Release 1.12.1 (338)
    Release1_12_1,
    /// Pre Release 1.12.2/2 (339)
    PreRelease1_12_2_PRE2,
    /// Release 1.12.2 (340)
    Release1_12_2,
    /// Snapshot 17w43a (341)
    Snapshot17W43A,
    /// Snapshot 17w43b (342)
    Snapshot17W43B,
    /// Snapshot 17w45a (343)
    Snapshot17W45A,
    /// Snapshot 17w45b (344)
    Snapshot17W45B,
    /// Snapshot 17w46a (345)
    Snapshot17W46A,
    /// Snapshot 17w47a (346)
    Snapshot17W47A,
    /// Snapshot 17w47b (347)
    Snapshot17W47B,
    /// Snapshot 17w48a (348)
    Snapshot17W48A,
    /// Snapshot 17w49a (349)
    Snapshot17W49A,
    /// Snapshot 17w49b (350)
    Snapshot17W49B,
    /// Snapshot 17w50a (351)
    Snapshot17W50A,
    /// Snapshot 18w01a (352)
    Snapshot18W01A,
    /// Snapshot 18w02a (353)
    Snapshot18W02A,
    /// Snapshot 18w03a (354)
    Snapshot18W03A,
    /// Snapshot 18w03b (355)
    Snapshot18W03B,
    /// Snapshot 18w05a (356)
    Snapshot18W05A,
    /// Snapshot 18w06a (357)
    Snapshot18W06A,
    /// Snapshot 18w07a (358)
    Snapshot18W07A,
    /// Snapshot 18w07b (359)
    Snapshot18W07B,
    /// Snapshot 18w07c (360)
    Snapshot18W07C,
    /// Snapshot 18w08a (361)
    Snapshot18W08A,
    /// Snapshot 18w08b (362)
    Snapshot18W08B,
    /// Snapshot 18w09a (363)
    Snapshot18W09A,
    /// Snapshot 18w10a (364)
    Snapshot18W10A,
    /// Snapshot 18w10b (365)
    Snapshot18W10B,
    /// Snapshot 18w10c (366)
    Snapshot18W10C,
    /// Snapshot 18w10d (367)
    Snapshot18W10D,
    /// Snapshot 18w11a (368)
    Snapshot18W11A,
    /// Snapshot 18w14a (369)
    Snapshot18W14A,
    /// Snapshot 18w14b (370)
    Snapshot18W14B,
    /// Snapshot 18w15a (371)
    Snapshot18W15A,
    /// Snapshot 18w16a (372)
    Snapshot18W16A,
    /// Snapshot 18w19a (373)
    Snapshot18W19A,
    /// Snapshot 18w19b (374)
    Snapshot18W19B,
    /// Snapshot 18w20a (375)
    Snapshot18W20A,
    /// Snapshot 18w20b (376)
    Snapshot18W20B,
    /// Snapshot 18w20c (377)
    Snapshot18W20C,
    /// Snapshot 18w21a (378)
    Snapshot18W21A,
    /// Snapshot 18w21b (379)
    Snapshot18W21B,
    /// Snapshot 18w22a (380)
    Snapshot18W22A,
    /// Snapshot 18w22b (381)
    Snapshot18W22B,
    /// Snapshot 18w22c (382)
    Snapshot18W22C,
    /// Pre Release 1.13/1 (383)
    PreRelease1_13_PRE1,
    /// Pre Release 1.13/2 (384)
    PreRelease1_13_PRE2,
    /// Pre Release 1.13/3 (385)
    PreRelease1_13_PRE3,
    /// Pre Release 1.13/4 (386)
    PreRelease1_13_PRE4,
    /// Pre Release 1.13/5 (387)
    PreRelease1_13_PRE5,
    /// Pre Release 1.13/6 (388)
    PreRelease1_13_PRE6,
    /// Pre Release 1.13/7 (389)
    PreRelease1_13_PRE7,
    /// Pre Release 1.13/8 (390)
    PreRelease1_13_PRE8,
    /// Pre Release 1.13/9 (391)
    PreRelease1_13_PRE9,
    /// Pre Release 1.13/10 (392)
    PreRelease1_13_PRE10,
    /// Release 1.13 (393)
    Release1_13,
    /// Snapshot 18w30a (394)
    Snapshot18W30A,
    /// Snapshot 18w30b (395)
    Snapshot18W30B,
    /// Snapshot 18w31a (396)
    Snapshot18W31A,
    /// Snapshot 18w32a (397)
    Snapshot18W32A,
    /// Snapshot 18w33a (398)
    Snapshot18W33A,
    /// Pre Release 1.13.1/1 (399)
    PreRelease1_13_1_PRE1,
    /// Pre Release 1.13.1/2 (400)
    PreRelease1_13_1_PRE2,
    /// Release 1.13.1 (401)
    Release1_13_1,
    /// Pre Release 1.13.2/1 (402)
    PreRelease1_13_2_PRE1,
    /// Pre Release 1.13.2/2 (403)
    PreRelease1_13_2_PRE2,
    /// Release 1.13.2 (404)
    Release1_13_2,
    /// Snapshot 18w43a (440)
    Snapshot18W43A,
    /// Snapshot 18w43b (441)
    Snapshot18W43B,
    /// Snapshot 18w43c (442)
    Snapshot18W43C,
    /// Snapshot 18w44a (443)
    Snapshot18W44A,
    /// Snapshot 18w45a (444)
    Snapshot18W45A,
    /// Snapshot 18w46a (445)
    Snapshot18W46A,
    /// Snapshot 18w47a (446)
    Snapshot18W47A,
    /// Snapshot 18w47b (447)
    Snapshot18W47B,
    /// Snapshot 18w48a (448)
    Snapshot18W48A,
    /// Snapshot 18w48b (449)
    Snapshot18W48B,
    /// Snapshot 18w49a (450)
    Snapshot18W49A,
    /// Snapshot 18w50a (451)
    Snapshot18W50A,
    /// Snapshot 19w02a (452)
    Snapshot19W02A,
    /// Snapshot 19w03a (453)
    Snapshot19W03A,
    /// Snapshot 19w03b (454)
    Snapshot19W03B,
    /// Snapshot 19w03c (455)
    Snapshot19W03C,
    /// Snapshot 19w04a (456)
    Snapshot19W04A,
    /// Snapshot 19w04b (457)
    Snapshot19W04B,
    /// Snapshot 19w05a (458)
    Snapshot19W05A,
    /// Snapshot 19w06a (459)
    Snapshot19W06A,
    /// Snapshot 19w07a (460)
    Snapshot19W07A,
    /// Snapshot 19w08a (461)
    Snapshot19W08A,
    /// Snapshot 19w08b (462)
    Snapshot19W08B,
    /// Snapshot 19w09a (463)
    Snapshot19W09A,
    /// Snapshot 19w11a (464)
    Snapshot19W11A,
    /// Snapshot 19w11b (465)
    Snapshot19W11B,
    /// Snapshot 19w12a (466)
    Snapshot19W12A,
    /// Snapshot 19w12b (467)
    Snapshot19W12B,
    /// Snapshot 19w13a (468)
    Snapshot19W13A,
    /// Snapshot 19w13b (469)
    Snapshot19W13B,
    /// Snapshot 19w14a (470)
    Snapshot19W14A,
    /// Snapshot 19w14b (471)
    Snapshot19W14B,
    /// Pre Release 1.14/1 (472)
    PreRelease1_14_PRE1,
    /// Pre Release 1.14/2 (473)
    PreRelease1_14_PRE2,
    /// Pre Release 1.14/3 (474)
    PreRelease1_14_PRE3,
    /// Pre Release 1.14/4 (475)
    PreRelease1_14_PRE4,
    /// Pre Release 1.14/5 (476)
    PreRelease1_14_PRE5,
    /// Release 1.14 (477)
    Release1_14,
    /// Pre Release 1.14.1/1 (478)
    PreRelease1_14_1_PRE1,
    /// Pre Release 1.14.1/2 (479)
    PreRelease1_14_1_PRE2,
    /// Release 1.14.1 (480)
    Release1_14_1,
    /// Pre Release 1.14.2/1 (481)
    PreRelease1_14_2_PRE1,
    /// Pre Release 1.14.2/2 (482)
    PreRelease1_14_2_PRE2,
    /// Pre Release 1.14.2/3 (483)
    PreRelease1_14_2_PRE3,
    /// Pre Release 1.14.2/4 (484)
    PreRelease1_14_2_PRE4,
    /// Release 1.14.2 (485)
    Release1_14_2,
    /// Pre Release 1.14.3/1 (486)
    PreRelease1_14_3_PRE1,
    /// Pre Release 1.14.3/2 (487)
    PreRelease1_14_3_PRE2,
    /// Pre Release 1.14.3/3 (488)
    PreRelease1_14_3_PRE3,
    /// Pre Release 1.14.3/4 (489)
    PreRelease1_14_3_PRE4,
    /// Release 1.14.3 (490)
    Release1_14_3,
    /// Pre Release 1.14.4/1 (491)
    PreRelease1_14_4_PRE1,
    /// Pre Release 1.14.4/2 (492)
    PreRelease1_14_4_PRE2,
    /// Pre Release 1.14.4/3 (493)
    PreRelease1_14_4_PRE3,
    /// Pre Release 1.14.4/4 (494)
    PreRelease1_14_4_PRE4,
    /// Pre Release 1.14.4/5 (495)
    PreRelease1_14_4_PRE5,
    /// Pre Release 1.14.4/6 (496)
    PreRelease1_14_4_PRE6,
    /// Pre Release 1.14.4/7 (497)
    PreRelease1_14_4_PRE7,
    /// Release 1.14.4 (498)
    Release1_14_4,
    /// Snapshot 19w34a (550)
    Snapshot19W34A,
    /// Snapshot 19w35a (551)
    Snapshot19W35A,
    /// Snapshot 19w36a (552)
    Snapshot19W36A,
    /// Snapshot 19w37a (553)
    Snapshot19W37A,
    /// Snapshot 19w38a (554)
    Snapshot19W38A,
    /// Snapshot 19w38b (555)
    Snapshot19W38B,
    /// Snapshot 19w39a (556)
    Snapshot19W39A,
    /// Snapshot 19w40a (557)
    Snapshot19W40A,
    /// Snapshot 19w41a (558)
    Snapshot19W41A,
    /// Snapshot 19w42a (559)
    Snapshot19W42A,
    /// Snapshot 19w44a (560)
    Snapshot19W44A,
    /// Snapshot 19w45a (561)
    Snapshot19W45A,
    /// Snapshot 19w45b (562)
    Snapshot19W45B,
    /// Snapshot 19w46a (563)
    Snapshot19W46A,
    /// Snapshot 19w46b (564)
    Snapshot19W46B,
    /// Pre Release 1.15/1 (565)
    PreRelease1_15_PRE1,
    /// Pre Release 1.15/2 (566)
    PreRelease1_15_PRE2,
    /// Pre Release 1.15/3 (567)
    PreRelease1_15_PRE3,
    /// Pre Release 1.15/4 (569)
    PreRelease1_15_PRE4,
    /// Pre Release 1.15/5 (570)
    PreRelease1_15_PRE5,
    /// Pre Release 1.15/6 (571)
    PreRelease1_15_PRE6,
    /// Pre Release 1.15/7 (572)
    PreRelease1_15_PRE7,
    /// Release 1.15 (573)
    Release1_15,
    /// Pre Release 1.15.1/1 (574)
    PreRelease1_15_1_PRE1,
    /// Release 1.15.1 (575)
    Release1_15_1,
    /// Pre Release 1.15.2/1 (576)
    PreRelease1_15_2_PRE1,
    /// Pre Release 1.15.2/2 (577)
    PreRelease1_15_2_PRE2,
    /// Release 1.15.2 (578)
    Release1_15_2,
    /// Snapshot 20w06a (701)
    Snapshot20W06A,
    /// Snapshot 20w07a (702)
    Snapshot20W07A,
    /// Snapshot 20w08a (703)
    Snapshot20W08A,
    /// Snapshot 20w09a (704)
    Snapshot20W09A,
    /// Snapshot 20w10a (705)
    Snapshot20W10A,
    /// Snapshot 20w11a (706)
    Snapshot20W11A,
    /// Snapshot 20w12a (707)
    Snapshot20W12A,
    /// Snapshot 20w13a (708)
    Snapshot20W13A,
    /// Snapshot 20w13b (709)
    Snapshot20W13B,
    /// Snapshot 20w14a (710)
    Snapshot20W14A,
    /// Snapshot 20w15a (711)
    Snapshot20W15A,
    /// Snapshot 20w16a (712)
    Snapshot20W16A,
    /// Snapshot 20w17a (713)
    Snapshot20W17A,
    /// Snapshot 20w18a (714)
    Snapshot20W18A,
    /// Snapshot 20w19a (715)
    Snapshot20W19A,
    /// Snapshot 20w20a (716)
    Snapshot20W20A,
    /// Snapshot 20w20b (717)
    Snapshot20W20B,
    /// Snapshot 20w21a (718)
    Snapshot20W21A,
    /// Snapshot 20w22a (719)
    Snapshot20W22A,
    /// Pre Release 1.16/1 (721)
    PreRelease1_16_PRE1,
    /// Pre Release 1.16/2 (722)
    PreRelease1_16_PRE2,
    /// Pre Release 1.16/3 (725)
    PreRelease1_16_PRE3,
    /// Pre Release 1.16/4 (727)
    PreRelease1_16_PRE4,
    /// Pre Release 1.16/5 (729)
    PreRelease1_16_PRE5,
    /// Pre Release 1.16/6 (730)
    PreRelease1_16_PRE6,
    /// Pre Release 1.16/7 (732)
    PreRelease1_16_PRE7,
    /// Pre Release 1.16/8 (733)
    PreRelease1_16_PRE8,
    /// Release Candidate 1.16/1 (734)
    ReleaseCandidate1_16_RC1,
    /// Release 1.16 (735)
    Release1_16,
    /// Release 1.16.1 (736)
    Release1_16_1,
    /// Snapshot 20w27a (738)
    Snapshot20W27A,
    /// Snapshot 20w28a (740)
    Snapshot20W28A,
    /// Snapshot 20w29a (741)
    Snapshot20W29A,
    /// Snapshot 20w30a (743)
    Snapshot20W30A,
    /// Pre Release 1.16.2/1 (744)
    PreRelease1_16_2_PRE1,
    /// Pre Release 1.16.2/2 (746)
    PreRelease1_16_2_PRE2,
    /// Pre Release 1.16.2/3 (748)
    PreRelease1_16_2_PRE3,
    /// Release Candidate 1.16.2/1 (749)
    ReleaseCandidate1_16_2_RC1,
    /// Release Candidate 1.16.2/2 (750)
    ReleaseCandidate1_16_2_RC2,
    /// Release 1.16.2 (751)
    Release1_16_2,
    /// Release Candidate 1.16.3/1 (752)
    ReleaseCandidate1_16_3_RC1,
    /// Release 1.16.3 (753)
    Release1_16_3,
    /// Pre Release 1.16.4/1 (754)
    PreRelease1_16_4_PRE1,
    /// Pre Release 1.16.4/2 (755)
    PreRelease1_16_4_PRE2,
    /// Release Candidate 1.16.4/1 (756)
    ReleaseCandidate1_16_4_RC1,
    /// Release Candidate 1.16.5/1 (757)
    ReleaseCandidate1_16_5_RC1,
    /// Release 1.16.5 (758)
    Release1_16_5,
    /// Snapshot 20w45a (759)
    Snapshot20W45A,
    /// Snapshot 20w46a (760)
    Snapshot20W46A,
    /// Snapshot 20w48a (761)
    Snapshot20W48A,
    /// Snapshot 20w49a (762)
    Snapshot20W49A,
    /// Snapshot 20w51a (763)
    Snapshot20W51A,
    /// Snapshot 21w03a (764)
    Snapshot21W03A,
    /// Snapshot 21w05a (765)
    Snapshot21W05A,
    /// Snapshot 21w05b (766)
    Snapshot21W05B,
    /// Snapshot 21w06a (767)
    Snapshot21W06A,
    /// Snapshot 21w07a (768)
    Snapshot21W07A,
    /// Snapshot 21w08a (769)
    Snapshot21W08A,
    /// Snapshot 21w08b (770)
    Snapshot21W08B,
    /// Snapshot 21w10a (771)
    Snapshot21W10A,
    /// Snapshot 21w11a (772)
    Snapshot21W11A,
    /// Snapshot 21w13a (773)
    Snapshot21W13A,
    /// Snapshot 21w14a (774)
    Snapshot21W14A,
    /// Snapshot 21w15a (775)
    Snapshot21W15A,
    /// Snapshot 21w16a (776)
    Snapshot21W16A,
    /// Snapshot 21w17a (777)
    Snapshot21W17A,
    /// Snapshot 21w18a (778)
    Snapshot21W18A,
    /// Snapshot 21w19a (779)
    Snapshot21W19A,
    /// Snapshot 21w20a (780)
    Snapshot21W20A,
    /// Pre Release 1.17/1 (781)
    PreRelease1_17_PRE1,
    /// Pre Release 1.17/2 (782)
    PreRelease1_17_PRE2,
    /// Pre Release 1.17/3 (783)
    PreRelease1_17_PRE3,
    /// Pre Release 1.17/4 (784)
    PreRelease1_17_PRE4,
    /// Pre Release 1.17/5 (785)
    PreRelease1_17_PRE5,
    /// Release Candidate 1.17/1 (786)
    ReleaseCandidate1_17_RC1,
    /// Release Candidate 1.17/2 (787)
    ReleaseCandidate1_17_RC2,
    /// Release 1.17 (788)
    Release1_17,
    /// Pre Release 1.17.1/1 (789)
    PreRelease1_17_1_PRE1,
    /// Pre Release 1.17.1/2 (790)
    PreRelease1_17_1_PRE2,
    /// Pre Release 1.17.1/3 (791)
    PreRelease1_17_1_PRE3,
    /// Release Candidate 1.17.1/1 (792)
    ReleaseCandidate1_17_1_RC1,
    /// Release Candidate 1.17.1/2 (793)
    ReleaseCandidate1_17_1_RC2,
    /// Release 1.17.1 (794)
    Release1_17_1,
    /// Snapshot 21w37a (795)
    Snapshot21W37A,
    /// Snapshot 21w38a (796)
    Snapshot21W38A,
    /// Snapshot 21w39a (797)
    Snapshot21W39A,
    /// Snapshot 21w40a (798)
    Snapshot21W40A,
    /// Snapshot 21w41a (799)
    Snapshot21W41A,
    /// Snapshot 21w42a (800)
    Snapshot21W42A,
    /// Snapshot 21w43a (801)
    Snapshot21W43A,
    /// Snapshot 21w44a (802)
    Snapshot21W44A,
    /// Pre Release 1.18/1 (803)
    PreRelease1_18_PRE1,
    /// Pre Release 1.18/2 (804)
    PreRelease1_18_PRE2,
    /// Pre Release 1.18/3 (805)
    PreRelease1_18_PRE3,
    /// Pre Release 1.18/4 (806)
    PreRelease1_18_PRE4,
    /// Pre Release 1.18/5 (807)
    PreRelease1_18_PRE5,
    /// Pre Release 1.18/6 (808)
    PreRelease1_18_PRE6,
    /// Pre Release 1.18/7 (809)
    PreRelease1_18_PRE7,
    /// Pre Release 1.18/8 (810)
    PreRelease1_18_PRE8,
    /// Release Candidate 1.18/1 (811)
    ReleaseCandidate1_18_RC1,
    /// Release Candidate 1.18/2 (812)
    ReleaseCandidate1_18_RC2,
    /// Release Candidate 1.18/3 (813)
    ReleaseCandidate1_18_RC3,
    /// Release Candidate 1.18/4 (814)
    ReleaseCandidate1_18_RC4,
    /// Pre Release 1.18.1/1 (816)
    PreRelease1_18_1_PRE1,
    /// Release Candidate 1.18.1/1 (817)
    ReleaseCandidate1_18_1_RC1,
    /// Release Candidate 1.18.1/2 (818)
    ReleaseCandidate1_18_1_RC2,
    /// Release Candidate 1.18.1/3 (819)
    ReleaseCandidate1_18_1_RC3,
    /// Release 1.18.1 (820)
    Release1_18_1,
    /// Snapshot 22w03a (821)
    Snapshot22W03A,
    /// Snapshot 22w05a (822)
    Snapshot22W05A,
    /// Snapshot 22w06a (823)
    Snapshot22W06A,
    /// Snapshot 22w07a (824)
    Snapshot22W07A,
    /// Pre Release 1.18.2/1 (825)
    PreRelease1_18_2_PRE1,
    /// Pre Release 1.18.2/2 (826)
    PreRelease1_18_2_PRE2,
    /// Pre Release 1.18.2/3 (827)
    PreRelease1_18_2_PRE3,
    /// Release Candidate 1.18.2/1 (828)
    ReleaseCandidate1_18_2_RC1,
    /// Release 1.18.2 (829)
    Release1_18_2,
    /// Snapshot 22w11a (830)
    Snapshot22W11A,
    /// Snapshot 22w12a (831)
    Snapshot22W12A,
    /// Snapshot 22w13a (832)
    Snapshot22W13A,
    /// Snapshot 22w13oneBlockAtATime (833)
    Snapshot22W13ONEBLOCKATATIME,
    /// Snapshot 22w14a (834)
    Snapshot22W14A,
    /// Snapshot 22w15a (835)
    Snapshot22W15A,
    /// Snapshot 22w16a (836)
    Snapshot22W16A,
    /// Snapshot 22w16b (837)
    Snapshot22W16B,
    /// Snapshot 22w17a (838)
    Snapshot22W17A,
    /// Snapshot 22w18a (839)
    Snapshot22W18A,
    /// Snapshot 22w19a (840)
    Snapshot22W19A,
    /// Pre Release 1.19/1 (841)
    PreRelease1_19_PRE1,
    /// Pre Release 1.19/2 (842)
    PreRelease1_19_PRE2,
    /// Pre Release 1.19/3 (843)
    PreRelease1_19_PRE3,
    /// Pre Release 1.19/4 (844)
    PreRelease1_19_PRE4,
    /// Pre Release 1.19/5 (845)
    PreRelease1_19_PRE5,
    /// Release Candidate 1.19/1 (846)
    ReleaseCandidate1_19_RC1,
    /// Release Candidate 1.19/2 (847)
    ReleaseCandidate1_19_RC2,
    /// Release 1.19 (848)
    Release1_19,
    /// Snapshot 22w24a (849)
    Snapshot22W24A,
    /// Pre Release 1.19.1/1 (850)
    PreRelease1_19_1_PRE1,
    /// Release Candidate 1.19.1/1 (851)
    ReleaseCandidate1_19_1_RC1,
    /// Pre Release 1.19.1/2 (852)
    PreRelease1_19_1_PRE2,
    /// Pre Release 1.19.1/3 (853)
    PreRelease1_19_1_PRE3,
    /// Pre Release 1.19.1/4 (854)
    PreRelease1_19_1_PRE4,
    /// Pre Release 1.19.1/5 (855)
    PreRelease1_19_1_PRE5,
    /// Pre Release 1.19.1/6 (856)
    PreRelease1_19_1_PRE6,
    /// Release Candidate 1.19.1/2 (857)
    ReleaseCandidate1_19_1_RC2,
    /// Release Candidate 1.19.1/3 (858)
    ReleaseCandidate1_19_1_RC3,
    /// Release Candidate 1.19.2/1 (860)
    ReleaseCandidate1_19_2_RC1,
    /// Release Candidate 1.19.2/2 (861)
    ReleaseCandidate1_19_2_RC2,
    /// Release 1.19.2 (862)
    Release1_19_2,
    /// Snapshot 22w42a (863)
    Snapshot22W42A,
    /// Snapshot 22w43a (864)
    Snapshot22W43A,
    /// Snapshot 22w44a (865)
    Snapshot22W44A,
    /// Snapshot 22w45a (866)
    Snapshot22W45A,
    /// Snapshot 22w46a (867)
    Snapshot22W46A,
    /// Pre Release 1.19.3/1 (868)
    PreRelease1_19_3_PRE1,
    /// Pre Release 1.19.3/2 (869)
    PreRelease1_19_3_PRE2,
    /// Pre Release 1.19.3/3 (870)
    PreRelease1_19_3_PRE3,
    /// Release Candidate 1.19.3/1 (871)
    ReleaseCandidate1_19_3_RC1,
    /// Release Candidate 1.19.3/2 (872)
    ReleaseCandidate1_19_3_RC2,
    /// Release Candidate 1.19.3/3 (873)
    ReleaseCandidate1_19_3_RC3,
    /// Release 1.19.3 (874)
    Release1_19_3,
    /// Snapshot 23w03a (875)
    Snapshot23W03A,
    /// Snapshot 23w04a (876)
    Snapshot23W04A,
    /// Snapshot 23w05a (877)
    Snapshot23W05A,
    /// Snapshot 23w06a (878)
    Snapshot23W06A,
    /// Snapshot 23w07a (879)
    Snapshot23W07A,
    /// Pre Release 1.19.4/1 (880)
    PreRelease1_19_4_PRE1,
    /// Pre Release 1.19.4/2 (881)
    PreRelease1_19_4_PRE2,
    /// Pre Release 1.19.4/3 (882)
    PreRelease1_19_4_PRE3,
    /// Pre Release 1.19.4/4 (883)
    PreRelease1_19_4_PRE4,
    /// Release Candidate 1.19.4/1 (884)
    ReleaseCandidate1_19_4_RC1,
    /// Release Candidate 1.19.4/2 (885)
    ReleaseCandidate1_19_4_RC2,
    /// Release Candidate 1.19.4/3 (886)
    ReleaseCandidate1_19_4_RC3,
    /// Release 1.19.4 (887)
    Release1_19_4,
    /// Snapshot 23w12a (888)
    Snapshot23W12A,
    /// Snapshot 23w13a (889)
    Snapshot23W13A,
    /// Snapshot 23w14a (890)
    Snapshot23W14A,
    /// Snapshot 23w16a (891)
    Snapshot23W16A,
    /// Snapshot 23w17a (892)
    Snapshot23W17A,
    /// Snapshot 23w18a (893)
    Snapshot23W18A,
    /// Pre Release 1.20/1 (894)
    PreRelease1_20_PRE1,
    /// Pre Release 1.20/2 (895)
    PreRelease1_20_PRE2,
    /// Pre Release 1.20/3 (896)
    PreRelease1_20_PRE3,
    /// Pre Release 1.20/4 (897)
    PreRelease1_20_PRE4,
    /// Pre Release 1.20/5 (898)
    PreRelease1_20_PRE5,
    /// Pre Release 1.20/6 (899)
    PreRelease1_20_PRE6,
    /// Pre Release 1.20/7 (900)
    PreRelease1_20_PRE7,
    /// Release Candidate 1.20/1 (901)
    ReleaseCandidate1_20_RC1,
    /// Release Candidate 1.20.1/1 (903)
    ReleaseCandidate1_20_1_RC1,
    /// Release 1.20.1 (904)
    Release1_20_1,
    /// Snapshot 23w31a (905)
    Snapshot23W31A,
    /// Snapshot 23w32a (906)
    Snapshot23W32A,
    /// Snapshot 23w33a (907)
    Snapshot23W33A,
    /// Snapshot 23w35a (908)
    Snapshot23W35A,
    /// Pre Release 1.20.2/1 (909)
    PreRelease1_20_2_PRE1,
    /// Pre Release 1.20.2/2 (910)
    PreRelease1_20_2_PRE2,
    /// Pre Release 1.20.2/3 (911)
    PreRelease1_20_2_PRE3,
    /// Pre Release 1.20.2/4 (912)
    PreRelease1_20_2_PRE4,
    /// Release Candidate 1.20.2/1 (913)
    ReleaseCandidate1_20_2_RC1,
    /// Release Candidate 1.20.2/2 (914)
    ReleaseCandidate1_20_2_RC2,
    /// Release 1.20.2 (915)
    Release1_20_2,
    /// Snapshot 23w40a (916)
    Snapshot23W40A,
    /// Snapshot 23w41a (917)
    Snapshot23W41A,
    /// Snapshot 23w42a (918)
    Snapshot23W42A,
    /// Snapshot 23w43a (919)
    Snapshot23W43A,
    /// Snapshot 23w43b (920)
    Snapshot23W43B,
    /// Snapshot 23w44a (921)
    Snapshot23W44A,
    /// Snapshot 23w45a (922)
    Snapshot23W45A,
    /// Snapshot 23w46a (923)
    Snapshot23W46A,
    /// Pre Release 1.20.3/1 (924)
    PreRelease1_20_3_PRE1,
    /// Pre Release 1.20.3/2 (925)
    PreRelease1_20_3_PRE2,
    /// Pre Release 1.20.3/3 (926)
    PreRelease1_20_3_PRE3,
    /// Pre Release 1.20.3/4 (927)
    PreRelease1_20_3_PRE4,
    /// Release Candidate 1.20.3/1 (928)
    ReleaseCandidate1_20_3_RC1,
    /// Release Candidate 1.20.4/1 (930)
    ReleaseCandidate1_20_4_RC1,
    /// Release 1.20.4 (931)
    Release1_20_4,
}

impl Version {
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn protocol_version_number(&self) -> VarInt {
        match self {
            Self::Snapshot13W41B => VarInt::from(0),
            Self::Snapshot13W42B => VarInt::from(1),
            Self::Snapshot13W43A => VarInt::from(2),
            Self::PreRelease1_7_1_PRE => VarInt::from(3),
            Self::Release1_7_5 => VarInt::from(4),
            Self::Release1_7_10 => VarInt::from(5),
            Self::Snapshot14W03B => VarInt::from(6),
            Self::Snapshot14W04A => VarInt::from(7),
            Self::Snapshot14W04B => VarInt::from(8),
            Self::Snapshot14W05B => VarInt::from(9),
            Self::Snapshot14W06B => VarInt::from(10),
            Self::Snapshot14W07A => VarInt::from(11),
            Self::Snapshot14W08A => VarInt::from(12),
            Self::Snapshot14W10C => VarInt::from(13),
            Self::Snapshot14W11B => VarInt::from(14),
            Self::Snapshot14W17A => VarInt::from(15),
            Self::Snapshot14W18B => VarInt::from(16),
            Self::Snapshot14W19A => VarInt::from(17),
            Self::Snapshot14W20B => VarInt::from(18),
            Self::Snapshot14W21A => VarInt::from(19),
            Self::Snapshot14W21B => VarInt::from(20),
            Self::Snapshot14W25A => VarInt::from(21),
            Self::Snapshot14W25B => VarInt::from(22),
            Self::Snapshot14W26A => VarInt::from(23),
            Self::Snapshot14W26B => VarInt::from(24),
            Self::Snapshot14W26C => VarInt::from(25),
            Self::Snapshot14W27B => VarInt::from(26),
            Self::Snapshot14W28A => VarInt::from(27),
            Self::Snapshot14W28B => VarInt::from(28),
            Self::Snapshot14W29A => VarInt::from(29),
            Self::Snapshot14W30B => VarInt::from(30),
            Self::Snapshot14W30C => VarInt::from(31),
            Self::Snapshot14W31A => VarInt::from(32),
            Self::Snapshot14W32A => VarInt::from(33),
            Self::Snapshot14W32B => VarInt::from(34),
            Self::Snapshot14W32C => VarInt::from(35),
            Self::Snapshot14W32D => VarInt::from(36),
            Self::Snapshot14W33A => VarInt::from(37),
            Self::Snapshot14W33B => VarInt::from(38),
            Self::Snapshot14W33C => VarInt::from(39),
            Self::Snapshot14W34A => VarInt::from(40),
            Self::Snapshot14W34B => VarInt::from(41),
            Self::Snapshot14W34C => VarInt::from(42),
            Self::Snapshot14W34D => VarInt::from(43),
            Self::PreRelease1_8_PRE1 => VarInt::from(44),
            Self::PreRelease1_8_PRE2 => VarInt::from(45),
            Self::PreRelease1_8_PRE3 => VarInt::from(46),
            Self::Release1_8_9 => VarInt::from(47),
            Self::Snapshot15W31A => VarInt::from(49),
            Self::Snapshot15W31B => VarInt::from(50),
            Self::Snapshot15W31C => VarInt::from(51),
            Self::Snapshot15W32A => VarInt::from(52),
            Self::Snapshot15W32B => VarInt::from(53),
            Self::Snapshot15W32C => VarInt::from(54),
            Self::Snapshot15W33A => VarInt::from(55),
            Self::Snapshot15W33B => VarInt::from(56),
            Self::Snapshot15W33C => VarInt::from(57),
            Self::Snapshot15W34A => VarInt::from(58),
            Self::Snapshot15W34B => VarInt::from(59),
            Self::Snapshot15W34C => VarInt::from(60),
            Self::Snapshot15W34D => VarInt::from(61),
            Self::Snapshot15W35A => VarInt::from(62),
            Self::Snapshot15W35B => VarInt::from(63),
            Self::Snapshot15W35C => VarInt::from(64),
            Self::Snapshot15W35D => VarInt::from(65),
            Self::Snapshot15W35E => VarInt::from(66),
            Self::Snapshot15W36A => VarInt::from(67),
            Self::Snapshot15W36B => VarInt::from(68),
            Self::Snapshot15W36C => VarInt::from(69),
            Self::Snapshot15W36D => VarInt::from(70),
            Self::Snapshot15W37A => VarInt::from(71),
            Self::Snapshot15W38A => VarInt::from(72),
            Self::Snapshot15W38B => VarInt::from(73),
            Self::Snapshot15W39C => VarInt::from(74),
            Self::Snapshot15W40A => VarInt::from(75),
            Self::Snapshot15W40B => VarInt::from(76),
            Self::Snapshot15W41A => VarInt::from(77),
            Self::Snapshot15W41B => VarInt::from(78),
            Self::Snapshot15W42A => VarInt::from(79),
            Self::Snapshot15W43A => VarInt::from(80),
            Self::Snapshot15W43B => VarInt::from(81),
            Self::Snapshot15W43C => VarInt::from(82),
            Self::Snapshot15W44A => VarInt::from(83),
            Self::Snapshot15W44B => VarInt::from(84),
            Self::Snapshot15W45A => VarInt::from(85),
            Self::Snapshot15W46A => VarInt::from(86),
            Self::Snapshot15W47A => VarInt::from(87),
            Self::Snapshot15W47B => VarInt::from(88),
            Self::Snapshot15W47C => VarInt::from(89),
            Self::Snapshot15W49A => VarInt::from(90),
            Self::Snapshot15W49B => VarInt::from(91),
            Self::Snapshot15W50A => VarInt::from(92),
            Self::Snapshot15W51A => VarInt::from(93),
            Self::Snapshot15W51B => VarInt::from(94),
            Self::Snapshot16W02A => VarInt::from(95),
            Self::Snapshot16W03A => VarInt::from(96),
            Self::Snapshot16W04A => VarInt::from(97),
            Self::Snapshot16W05A => VarInt::from(98),
            Self::Snapshot16W05B => VarInt::from(99),
            Self::Snapshot16W06A => VarInt::from(100),
            Self::Snapshot16W07A => VarInt::from(101),
            Self::Snapshot16W07B => VarInt::from(102),
            Self::PreRelease1_9_PRE1 => VarInt::from(103),
            Self::PreRelease1_9_PRE2 => VarInt::from(104),
            Self::PreRelease1_9_PRE3 => VarInt::from(105),
            Self::PreRelease1_9_PRE4 => VarInt::from(106),
            Self::PreRelease1_9_1_PRE1 => VarInt::from(107),
            Self::Release1_9_1 => VarInt::from(108),
            Self::PreRelease1_9_3_PRE1 => VarInt::from(109),
            Self::Release1_9_4 => VarInt::from(110),
            Self::Snapshot16W20A => VarInt::from(201),
            Self::Snapshot16W21A => VarInt::from(202),
            Self::Snapshot16W21B => VarInt::from(203),
            Self::PreRelease1_10_PRE1 => VarInt::from(204),
            Self::PreRelease1_10_PRE2 => VarInt::from(205),
            Self::Release1_10_2 => VarInt::from(210),
            Self::Snapshot16W32A => VarInt::from(301),
            Self::Snapshot16W32B => VarInt::from(302),
            Self::Snapshot16W33A => VarInt::from(303),
            Self::Snapshot16W35A => VarInt::from(304),
            Self::Snapshot16W36A => VarInt::from(305),
            Self::Snapshot16W38A => VarInt::from(306),
            Self::Snapshot16W39A => VarInt::from(307),
            Self::Snapshot16W39B => VarInt::from(308),
            Self::Snapshot16W39C => VarInt::from(309),
            Self::Snapshot16W40A => VarInt::from(310),
            Self::Snapshot16W41A => VarInt::from(311),
            Self::Snapshot16W42A => VarInt::from(312),
            Self::Snapshot16W44A => VarInt::from(313),
            Self::PreRelease1_11_PRE1 => VarInt::from(314),
            Self::Release1_11 => VarInt::from(315),
            Self::Release1_11_2 => VarInt::from(316),
            Self::Snapshot17W06A => VarInt::from(317),
            Self::Snapshot17W13A => VarInt::from(318),
            Self::Snapshot17W13B => VarInt::from(319),
            Self::Snapshot17W14A => VarInt::from(320),
            Self::Snapshot17W15A => VarInt::from(321),
            Self::Snapshot17W16A => VarInt::from(322),
            Self::Snapshot17W16B => VarInt::from(323),
            Self::Snapshot17W17A => VarInt::from(324),
            Self::Snapshot17W17B => VarInt::from(325),
            Self::Snapshot17W18A => VarInt::from(326),
            Self::Snapshot17W18B => VarInt::from(327),
            Self::PreRelease1_12_PRE1 => VarInt::from(328),
            Self::PreRelease1_12_PRE2 => VarInt::from(329),
            Self::PreRelease1_12_PRE3 => VarInt::from(330),
            Self::PreRelease1_12_PRE4 => VarInt::from(331),
            Self::PreRelease1_12_PRE5 => VarInt::from(332),
            Self::PreRelease1_12_PRE6 => VarInt::from(333),
            Self::PreRelease1_12_PRE7 => VarInt::from(334),
            Self::Release1_12 => VarInt::from(335),
            Self::Snapshot17W31A => VarInt::from(336),
            Self::PreRelease1_12_1_PRE1 => VarInt::from(337),
            Self::Release1_12_1 => VarInt::from(338),
            Self::PreRelease1_12_2_PRE2 => VarInt::from(339),
            Self::Release1_12_2 => VarInt::from(340),
            Self::Snapshot17W43A => VarInt::from(341),
            Self::Snapshot17W43B => VarInt::from(342),
            Self::Snapshot17W45A => VarInt::from(343),
            Self::Snapshot17W45B => VarInt::from(344),
            Self::Snapshot17W46A => VarInt::from(345),
            Self::Snapshot17W47A => VarInt::from(346),
            Self::Snapshot17W47B => VarInt::from(347),
            Self::Snapshot17W48A => VarInt::from(348),
            Self::Snapshot17W49A => VarInt::from(349),
            Self::Snapshot17W49B => VarInt::from(350),
            Self::Snapshot17W50A => VarInt::from(351),
            Self::Snapshot18W01A => VarInt::from(352),
            Self::Snapshot18W02A => VarInt::from(353),
            Self::Snapshot18W03A => VarInt::from(354),
            Self::Snapshot18W03B => VarInt::from(355),
            Self::Snapshot18W05A => VarInt::from(356),
            Self::Snapshot18W06A => VarInt::from(357),
            Self::Snapshot18W07A => VarInt::from(358),
            Self::Snapshot18W07B => VarInt::from(359),
            Self::Snapshot18W07C => VarInt::from(360),
            Self::Snapshot18W08A => VarInt::from(361),
            Self::Snapshot18W08B => VarInt::from(362),
            Self::Snapshot18W09A => VarInt::from(363),
            Self::Snapshot18W10A => VarInt::from(364),
            Self::Snapshot18W10B => VarInt::from(365),
            Self::Snapshot18W10C => VarInt::from(366),
            Self::Snapshot18W10D => VarInt::from(367),
            Self::Snapshot18W11A => VarInt::from(368),
            Self::Snapshot18W14A => VarInt::from(369),
            Self::Snapshot18W14B => VarInt::from(370),
            Self::Snapshot18W15A => VarInt::from(371),
            Self::Snapshot18W16A => VarInt::from(372),
            Self::Snapshot18W19A => VarInt::from(373),
            Self::Snapshot18W19B => VarInt::from(374),
            Self::Snapshot18W20A => VarInt::from(375),
            Self::Snapshot18W20B => VarInt::from(376),
            Self::Snapshot18W20C => VarInt::from(377),
            Self::Snapshot18W21A => VarInt::from(378),
            Self::Snapshot18W21B => VarInt::from(379),
            Self::Snapshot18W22A => VarInt::from(380),
            Self::Snapshot18W22B => VarInt::from(381),
            Self::Snapshot18W22C => VarInt::from(382),
            Self::PreRelease1_13_PRE1 => VarInt::from(383),
            Self::PreRelease1_13_PRE2 => VarInt::from(384),
            Self::PreRelease1_13_PRE3 => VarInt::from(385),
            Self::PreRelease1_13_PRE4 => VarInt::from(386),
            Self::PreRelease1_13_PRE5 => VarInt::from(387),
            Self::PreRelease1_13_PRE6 => VarInt::from(388),
            Self::PreRelease1_13_PRE7 => VarInt::from(389),
            Self::PreRelease1_13_PRE8 => VarInt::from(390),
            Self::PreRelease1_13_PRE9 => VarInt::from(391),
            Self::PreRelease1_13_PRE10 => VarInt::from(392),
            Self::Release1_13 => VarInt::from(393),
            Self::Snapshot18W30A => VarInt::from(394),
            Self::Snapshot18W30B => VarInt::from(395),
            Self::Snapshot18W31A => VarInt::from(396),
            Self::Snapshot18W32A => VarInt::from(397),
            Self::Snapshot18W33A => VarInt::from(398),
            Self::PreRelease1_13_1_PRE1 => VarInt::from(399),
            Self::PreRelease1_13_1_PRE2 => VarInt::from(400),
            Self::Release1_13_1 => VarInt::from(401),
            Self::PreRelease1_13_2_PRE1 => VarInt::from(402),
            Self::PreRelease1_13_2_PRE2 => VarInt::from(403),
            Self::Release1_13_2 => VarInt::from(404),
            Self::Snapshot18W43A => VarInt::from(440),
            Self::Snapshot18W43B => VarInt::from(441),
            Self::Snapshot18W43C => VarInt::from(442),
            Self::Snapshot18W44A => VarInt::from(443),
            Self::Snapshot18W45A => VarInt::from(444),
            Self::Snapshot18W46A => VarInt::from(445),
            Self::Snapshot18W47A => VarInt::from(446),
            Self::Snapshot18W47B => VarInt::from(447),
            Self::Snapshot18W48A => VarInt::from(448),
            Self::Snapshot18W48B => VarInt::from(449),
            Self::Snapshot18W49A => VarInt::from(450),
            Self::Snapshot18W50A => VarInt::from(451),
            Self::Snapshot19W02A => VarInt::from(452),
            Self::Snapshot19W03A => VarInt::from(453),
            Self::Snapshot19W03B => VarInt::from(454),
            Self::Snapshot19W03C => VarInt::from(455),
            Self::Snapshot19W04A => VarInt::from(456),
            Self::Snapshot19W04B => VarInt::from(457),
            Self::Snapshot19W05A => VarInt::from(458),
            Self::Snapshot19W06A => VarInt::from(459),
            Self::Snapshot19W07A => VarInt::from(460),
            Self::Snapshot19W08A => VarInt::from(461),
            Self::Snapshot19W08B => VarInt::from(462),
            Self::Snapshot19W09A => VarInt::from(463),
            Self::Snapshot19W11A => VarInt::from(464),
            Self::Snapshot19W11B => VarInt::from(465),
            Self::Snapshot19W12A => VarInt::from(466),
            Self::Snapshot19W12B => VarInt::from(467),
            Self::Snapshot19W13A => VarInt::from(468),
            Self::Snapshot19W13B => VarInt::from(469),
            Self::Snapshot19W14A => VarInt::from(470),
            Self::Snapshot19W14B => VarInt::from(471),
            Self::PreRelease1_14_PRE1 => VarInt::from(472),
            Self::PreRelease1_14_PRE2 => VarInt::from(473),
            Self::PreRelease1_14_PRE3 => VarInt::from(474),
            Self::PreRelease1_14_PRE4 => VarInt::from(475),
            Self::PreRelease1_14_PRE5 => VarInt::from(476),
            Self::Release1_14 => VarInt::from(477),
            Self::PreRelease1_14_1_PRE1 => VarInt::from(478),
            Self::PreRelease1_14_1_PRE2 => VarInt::from(479),
            Self::Release1_14_1 => VarInt::from(480),
            Self::PreRelease1_14_2_PRE1 => VarInt::from(481),
            Self::PreRelease1_14_2_PRE2 => VarInt::from(482),
            Self::PreRelease1_14_2_PRE3 => VarInt::from(483),
            Self::PreRelease1_14_2_PRE4 => VarInt::from(484),
            Self::Release1_14_2 => VarInt::from(485),
            Self::PreRelease1_14_3_PRE1 => VarInt::from(486),
            Self::PreRelease1_14_3_PRE2 => VarInt::from(487),
            Self::PreRelease1_14_3_PRE3 => VarInt::from(488),
            Self::PreRelease1_14_3_PRE4 => VarInt::from(489),
            Self::Release1_14_3 => VarInt::from(490),
            Self::PreRelease1_14_4_PRE1 => VarInt::from(491),
            Self::PreRelease1_14_4_PRE2 => VarInt::from(492),
            Self::PreRelease1_14_4_PRE3 => VarInt::from(493),
            Self::PreRelease1_14_4_PRE4 => VarInt::from(494),
            Self::PreRelease1_14_4_PRE5 => VarInt::from(495),
            Self::PreRelease1_14_4_PRE6 => VarInt::from(496),
            Self::PreRelease1_14_4_PRE7 => VarInt::from(497),
            Self::Release1_14_4 => VarInt::from(498),
            Self::Snapshot19W34A => VarInt::from(550),
            Self::Snapshot19W35A => VarInt::from(551),
            Self::Snapshot19W36A => VarInt::from(552),
            Self::Snapshot19W37A => VarInt::from(553),
            Self::Snapshot19W38A => VarInt::from(554),
            Self::Snapshot19W38B => VarInt::from(555),
            Self::Snapshot19W39A => VarInt::from(556),
            Self::Snapshot19W40A => VarInt::from(557),
            Self::Snapshot19W41A => VarInt::from(558),
            Self::Snapshot19W42A => VarInt::from(559),
            Self::Snapshot19W44A => VarInt::from(560),
            Self::Snapshot19W45A => VarInt::from(561),
            Self::Snapshot19W45B => VarInt::from(562),
            Self::Snapshot19W46A => VarInt::from(563),
            Self::Snapshot19W46B => VarInt::from(564),
            Self::PreRelease1_15_PRE1 => VarInt::from(565),
            Self::PreRelease1_15_PRE2 => VarInt::from(566),
            Self::PreRelease1_15_PRE3 => VarInt::from(567),
            Self::PreRelease1_15_PRE4 => VarInt::from(569),
            Self::PreRelease1_15_PRE5 => VarInt::from(570),
            Self::PreRelease1_15_PRE6 => VarInt::from(571),
            Self::PreRelease1_15_PRE7 => VarInt::from(572),
            Self::Release1_15 => VarInt::from(573),
            Self::PreRelease1_15_1_PRE1 => VarInt::from(574),
            Self::Release1_15_1 => VarInt::from(575),
            Self::PreRelease1_15_2_PRE1 => VarInt::from(576),
            Self::PreRelease1_15_2_PRE2 => VarInt::from(577),
            Self::Release1_15_2 => VarInt::from(578),
            Self::Snapshot20W06A => VarInt::from(701),
            Self::Snapshot20W07A => VarInt::from(702),
            Self::Snapshot20W08A => VarInt::from(703),
            Self::Snapshot20W09A => VarInt::from(704),
            Self::Snapshot20W10A => VarInt::from(705),
            Self::Snapshot20W11A => VarInt::from(706),
            Self::Snapshot20W12A => VarInt::from(707),
            Self::Snapshot20W13A => VarInt::from(708),
            Self::Snapshot20W13B => VarInt::from(709),
            Self::Snapshot20W14A => VarInt::from(710),
            Self::Snapshot20W15A => VarInt::from(711),
            Self::Snapshot20W16A => VarInt::from(712),
            Self::Snapshot20W17A => VarInt::from(713),
            Self::Snapshot20W18A => VarInt::from(714),
            Self::Snapshot20W19A => VarInt::from(715),
            Self::Snapshot20W20A => VarInt::from(716),
            Self::Snapshot20W20B => VarInt::from(717),
            Self::Snapshot20W21A => VarInt::from(718),
            Self::Snapshot20W22A => VarInt::from(719),
            Self::PreRelease1_16_PRE1 => VarInt::from(721),
            Self::PreRelease1_16_PRE2 => VarInt::from(722),
            Self::PreRelease1_16_PRE3 => VarInt::from(725),
            Self::PreRelease1_16_PRE4 => VarInt::from(727),
            Self::PreRelease1_16_PRE5 => VarInt::from(729),
            Self::PreRelease1_16_PRE6 => VarInt::from(730),
            Self::PreRelease1_16_PRE7 => VarInt::from(732),
            Self::PreRelease1_16_PRE8 => VarInt::from(733),
            Self::ReleaseCandidate1_16_RC1 => VarInt::from(734),
            Self::Release1_16 => VarInt::from(735),
            Self::Release1_16_1 => VarInt::from(736),
            Self::Snapshot20W27A => VarInt::from(738),
            Self::Snapshot20W28A => VarInt::from(740),
            Self::Snapshot20W29A => VarInt::from(741),
            Self::Snapshot20W30A => VarInt::from(743),
            Self::PreRelease1_16_2_PRE1 => VarInt::from(744),
            Self::PreRelease1_16_2_PRE2 => VarInt::from(746),
            Self::PreRelease1_16_2_PRE3 => VarInt::from(748),
            Self::ReleaseCandidate1_16_2_RC1 => VarInt::from(749),
            Self::ReleaseCandidate1_16_2_RC2 => VarInt::from(750),
            Self::Release1_16_2 => VarInt::from(751),
            Self::ReleaseCandidate1_16_3_RC1 => VarInt::from(752),
            Self::Release1_16_3 => VarInt::from(753),
            Self::PreRelease1_16_4_PRE1 => VarInt::from(754),
            Self::PreRelease1_16_4_PRE2 => VarInt::from(755),
            Self::ReleaseCandidate1_16_4_RC1 => VarInt::from(756),
            Self::ReleaseCandidate1_16_5_RC1 => VarInt::from(757),
            Self::Release1_16_5 => VarInt::from(758),
            Self::Snapshot20W45A => VarInt::from(759),
            Self::Snapshot20W46A => VarInt::from(760),
            Self::Snapshot20W48A => VarInt::from(761),
            Self::Snapshot20W49A => VarInt::from(762),
            Self::Snapshot20W51A => VarInt::from(763),
            Self::Snapshot21W03A => VarInt::from(764),
            Self::Snapshot21W05A => VarInt::from(765),
            Self::Snapshot21W05B => VarInt::from(766),
            Self::Snapshot21W06A => VarInt::from(767),
            Self::Snapshot21W07A => VarInt::from(768),
            Self::Snapshot21W08A => VarInt::from(769),
            Self::Snapshot21W08B => VarInt::from(770),
            Self::Snapshot21W10A => VarInt::from(771),
            Self::Snapshot21W11A => VarInt::from(772),
            Self::Snapshot21W13A => VarInt::from(773),
            Self::Snapshot21W14A => VarInt::from(774),
            Self::Snapshot21W15A => VarInt::from(775),
            Self::Snapshot21W16A => VarInt::from(776),
            Self::Snapshot21W17A => VarInt::from(777),
            Self::Snapshot21W18A => VarInt::from(778),
            Self::Snapshot21W19A => VarInt::from(779),
            Self::Snapshot21W20A => VarInt::from(780),
            Self::PreRelease1_17_PRE1 => VarInt::from(781),
            Self::PreRelease1_17_PRE2 => VarInt::from(782),
            Self::PreRelease1_17_PRE3 => VarInt::from(783),
            Self::PreRelease1_17_PRE4 => VarInt::from(784),
            Self::PreRelease1_17_PRE5 => VarInt::from(785),
            Self::ReleaseCandidate1_17_RC1 => VarInt::from(786),
            Self::ReleaseCandidate1_17_RC2 => VarInt::from(787),
            Self::Release1_17 => VarInt::from(788),
            Self::PreRelease1_17_1_PRE1 => VarInt::from(789),
            Self::PreRelease1_17_1_PRE2 => VarInt::from(790),
            Self::PreRelease1_17_1_PRE3 => VarInt::from(791),
            Self::ReleaseCandidate1_17_1_RC1 => VarInt::from(792),
            Self::ReleaseCandidate1_17_1_RC2 => VarInt::from(793),
            Self::Release1_17_1 => VarInt::from(794),
            Self::Snapshot21W37A => VarInt::from(795),
            Self::Snapshot21W38A => VarInt::from(796),
            Self::Snapshot21W39A => VarInt::from(797),
            Self::Snapshot21W40A => VarInt::from(798),
            Self::Snapshot21W41A => VarInt::from(799),
            Self::Snapshot21W42A => VarInt::from(800),
            Self::Snapshot21W43A => VarInt::from(801),
            Self::Snapshot21W44A => VarInt::from(802),
            Self::PreRelease1_18_PRE1 => VarInt::from(803),
            Self::PreRelease1_18_PRE2 => VarInt::from(804),
            Self::PreRelease1_18_PRE3 => VarInt::from(805),
            Self::PreRelease1_18_PRE4 => VarInt::from(806),
            Self::PreRelease1_18_PRE5 => VarInt::from(807),
            Self::PreRelease1_18_PRE6 => VarInt::from(808),
            Self::PreRelease1_18_PRE7 => VarInt::from(809),
            Self::PreRelease1_18_PRE8 => VarInt::from(810),
            Self::ReleaseCandidate1_18_RC1 => VarInt::from(811),
            Self::ReleaseCandidate1_18_RC2 => VarInt::from(812),
            Self::ReleaseCandidate1_18_RC3 => VarInt::from(813),
            Self::ReleaseCandidate1_18_RC4 => VarInt::from(814),
            Self::PreRelease1_18_1_PRE1 => VarInt::from(816),
            Self::ReleaseCandidate1_18_1_RC1 => VarInt::from(817),
            Self::ReleaseCandidate1_18_1_RC2 => VarInt::from(818),
            Self::ReleaseCandidate1_18_1_RC3 => VarInt::from(819),
            Self::Release1_18_1 => VarInt::from(820),
            Self::Snapshot22W03A => VarInt::from(821),
            Self::Snapshot22W05A => VarInt::from(822),
            Self::Snapshot22W06A => VarInt::from(823),
            Self::Snapshot22W07A => VarInt::from(824),
            Self::PreRelease1_18_2_PRE1 => VarInt::from(825),
            Self::PreRelease1_18_2_PRE2 => VarInt::from(826),
            Self::PreRelease1_18_2_PRE3 => VarInt::from(827),
            Self::ReleaseCandidate1_18_2_RC1 => VarInt::from(828),
            Self::Release1_18_2 => VarInt::from(829),
            Self::Snapshot22W11A => VarInt::from(830),
            Self::Snapshot22W12A => VarInt::from(831),
            Self::Snapshot22W13A => VarInt::from(832),
            Self::Snapshot22W13ONEBLOCKATATIME => VarInt::from(833),
            Self::Snapshot22W14A => VarInt::from(834),
            Self::Snapshot22W15A => VarInt::from(835),
            Self::Snapshot22W16A => VarInt::from(836),
            Self::Snapshot22W16B => VarInt::from(837),
            Self::Snapshot22W17A => VarInt::from(838),
            Self::Snapshot22W18A => VarInt::from(839),
            Self::Snapshot22W19A => VarInt::from(840),
            Self::PreRelease1_19_PRE1 => VarInt::from(841),
            Self::PreRelease1_19_PRE2 => VarInt::from(842),
            Self::PreRelease1_19_PRE3 => VarInt::from(843),
            Self::PreRelease1_19_PRE4 => VarInt::from(844),
            Self::PreRelease1_19_PRE5 => VarInt::from(845),
            Self::ReleaseCandidate1_19_RC1 => VarInt::from(846),
            Self::ReleaseCandidate1_19_RC2 => VarInt::from(847),
            Self::Release1_19 => VarInt::from(848),
            Self::Snapshot22W24A => VarInt::from(849),
            Self::PreRelease1_19_1_PRE1 => VarInt::from(850),
            Self::ReleaseCandidate1_19_1_RC1 => VarInt::from(851),
            Self::PreRelease1_19_1_PRE2 => VarInt::from(852),
            Self::PreRelease1_19_1_PRE3 => VarInt::from(853),
            Self::PreRelease1_19_1_PRE4 => VarInt::from(854),
            Self::PreRelease1_19_1_PRE5 => VarInt::from(855),
            Self::PreRelease1_19_1_PRE6 => VarInt::from(856),
            Self::ReleaseCandidate1_19_1_RC2 => VarInt::from(857),
            Self::ReleaseCandidate1_19_1_RC3 => VarInt::from(858),
            Self::ReleaseCandidate1_19_2_RC1 => VarInt::from(860),
            Self::ReleaseCandidate1_19_2_RC2 => VarInt::from(861),
            Self::Release1_19_2 => VarInt::from(862),
            Self::Snapshot22W42A => VarInt::from(863),
            Self::Snapshot22W43A => VarInt::from(864),
            Self::Snapshot22W44A => VarInt::from(865),
            Self::Snapshot22W45A => VarInt::from(866),
            Self::Snapshot22W46A => VarInt::from(867),
            Self::PreRelease1_19_3_PRE1 => VarInt::from(868),
            Self::PreRelease1_19_3_PRE2 => VarInt::from(869),
            Self::PreRelease1_19_3_PRE3 => VarInt::from(870),
            Self::ReleaseCandidate1_19_3_RC1 => VarInt::from(871),
            Self::ReleaseCandidate1_19_3_RC2 => VarInt::from(872),
            Self::ReleaseCandidate1_19_3_RC3 => VarInt::from(873),
            Self::Release1_19_3 => VarInt::from(874),
            Self::Snapshot23W03A => VarInt::from(875),
            Self::Snapshot23W04A => VarInt::from(876),
            Self::Snapshot23W05A => VarInt::from(877),
            Self::Snapshot23W06A => VarInt::from(878),
            Self::Snapshot23W07A => VarInt::from(879),
            Self::PreRelease1_19_4_PRE1 => VarInt::from(880),
            Self::PreRelease1_19_4_PRE2 => VarInt::from(881),
            Self::PreRelease1_19_4_PRE3 => VarInt::from(882),
            Self::PreRelease1_19_4_PRE4 => VarInt::from(883),
            Self::ReleaseCandidate1_19_4_RC1 => VarInt::from(884),
            Self::ReleaseCandidate1_19_4_RC2 => VarInt::from(885),
            Self::ReleaseCandidate1_19_4_RC3 => VarInt::from(886),
            Self::Release1_19_4 => VarInt::from(887),
            Self::Snapshot23W12A => VarInt::from(888),
            Self::Snapshot23W13A => VarInt::from(889),
            Self::Snapshot23W14A => VarInt::from(890),
            Self::Snapshot23W16A => VarInt::from(891),
            Self::Snapshot23W17A => VarInt::from(892),
            Self::Snapshot23W18A => VarInt::from(893),
            Self::PreRelease1_20_PRE1 => VarInt::from(894),
            Self::PreRelease1_20_PRE2 => VarInt::from(895),
            Self::PreRelease1_20_PRE3 => VarInt::from(896),
            Self::PreRelease1_20_PRE4 => VarInt::from(897),
            Self::PreRelease1_20_PRE5 => VarInt::from(898),
            Self::PreRelease1_20_PRE6 => VarInt::from(899),
            Self::PreRelease1_20_PRE7 => VarInt::from(900),
            Self::ReleaseCandidate1_20_RC1 => VarInt::from(901),
            Self::ReleaseCandidate1_20_1_RC1 => VarInt::from(903),
            Self::Release1_20_1 => VarInt::from(904),
            Self::Snapshot23W31A => VarInt::from(905),
            Self::Snapshot23W32A => VarInt::from(906),
            Self::Snapshot23W33A => VarInt::from(907),
            Self::Snapshot23W35A => VarInt::from(908),
            Self::PreRelease1_20_2_PRE1 => VarInt::from(909),
            Self::PreRelease1_20_2_PRE2 => VarInt::from(910),
            Self::PreRelease1_20_2_PRE3 => VarInt::from(911),
            Self::PreRelease1_20_2_PRE4 => VarInt::from(912),
            Self::ReleaseCandidate1_20_2_RC1 => VarInt::from(913),
            Self::ReleaseCandidate1_20_2_RC2 => VarInt::from(914),
            Self::Release1_20_2 => VarInt::from(915),
            Self::Snapshot23W40A => VarInt::from(916),
            Self::Snapshot23W41A => VarInt::from(917),
            Self::Snapshot23W42A => VarInt::from(918),
            Self::Snapshot23W43A => VarInt::from(919),
            Self::Snapshot23W43B => VarInt::from(920),
            Self::Snapshot23W44A => VarInt::from(921),
            Self::Snapshot23W45A => VarInt::from(922),
            Self::Snapshot23W46A => VarInt::from(923),
            Self::PreRelease1_20_3_PRE1 => VarInt::from(924),
            Self::PreRelease1_20_3_PRE2 => VarInt::from(925),
            Self::PreRelease1_20_3_PRE3 => VarInt::from(926),
            Self::PreRelease1_20_3_PRE4 => VarInt::from(927),
            Self::ReleaseCandidate1_20_3_RC1 => VarInt::from(928),
            Self::ReleaseCandidate1_20_4_RC1 => VarInt::from(930),
            Self::Release1_20_4 => VarInt::from(931),
        }
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn name(&self) -> &'static str {
        match self {
            Self::Snapshot13W41B => "Snapshot 13w41b",
            Self::Snapshot13W42B => "Snapshot 13w42b",
            Self::Snapshot13W43A => "Snapshot 13w43a",
            Self::PreRelease1_7_1_PRE => "Pre Release 1.7.1",
            Self::Release1_7_5 => "Release 1.7.5",
            Self::Release1_7_10 => "Release 1.7.10",
            Self::Snapshot14W03B => "Snapshot 14w03b",
            Self::Snapshot14W04A => "Snapshot 14w04a",
            Self::Snapshot14W04B => "Snapshot 14w04b",
            Self::Snapshot14W05B => "Snapshot 14w05b",
            Self::Snapshot14W06B => "Snapshot 14w06b",
            Self::Snapshot14W07A => "Snapshot 14w07a",
            Self::Snapshot14W08A => "Snapshot 14w08a",
            Self::Snapshot14W10C => "Snapshot 14w10c",
            Self::Snapshot14W11B => "Snapshot 14w11b",
            Self::Snapshot14W17A => "Snapshot 14w17a",
            Self::Snapshot14W18B => "Snapshot 14w18b",
            Self::Snapshot14W19A => "Snapshot 14w19a",
            Self::Snapshot14W20B => "Snapshot 14w20b",
            Self::Snapshot14W21A => "Snapshot 14w21a",
            Self::Snapshot14W21B => "Snapshot 14w21b",
            Self::Snapshot14W25A => "Snapshot 14w25a",
            Self::Snapshot14W25B => "Snapshot 14w25b",
            Self::Snapshot14W26A => "Snapshot 14w26a",
            Self::Snapshot14W26B => "Snapshot 14w26b",
            Self::Snapshot14W26C => "Snapshot 14w26c",
            Self::Snapshot14W27B => "Snapshot 14w27b",
            Self::Snapshot14W28A => "Snapshot 14w28a",
            Self::Snapshot14W28B => "Snapshot 14w28b",
            Self::Snapshot14W29A => "Snapshot 14w29a",
            Self::Snapshot14W30B => "Snapshot 14w30b",
            Self::Snapshot14W30C => "Snapshot 14w30c",
            Self::Snapshot14W31A => "Snapshot 14w31a",
            Self::Snapshot14W32A => "Snapshot 14w32a",
            Self::Snapshot14W32B => "Snapshot 14w32b",
            Self::Snapshot14W32C => "Snapshot 14w32c",
            Self::Snapshot14W32D => "Snapshot 14w32d",
            Self::Snapshot14W33A => "Snapshot 14w33a",
            Self::Snapshot14W33B => "Snapshot 14w33b",
            Self::Snapshot14W33C => "Snapshot 14w33c",
            Self::Snapshot14W34A => "Snapshot 14w34a",
            Self::Snapshot14W34B => "Snapshot 14w34b",
            Self::Snapshot14W34C => "Snapshot 14w34c",
            Self::Snapshot14W34D => "Snapshot 14w34d",
            Self::PreRelease1_8_PRE1 => "Pre Release 1.8/1",
            Self::PreRelease1_8_PRE2 => "Pre Release 1.8/2",
            Self::PreRelease1_8_PRE3 => "Pre Release 1.8/3",
            Self::Release1_8_9 => "Release 1.8.9",
            Self::Snapshot15W31A => "Snapshot 15w31a",
            Self::Snapshot15W31B => "Snapshot 15w31b",
            Self::Snapshot15W31C => "Snapshot 15w31c",
            Self::Snapshot15W32A => "Snapshot 15w32a",
            Self::Snapshot15W32B => "Snapshot 15w32b",
            Self::Snapshot15W32C => "Snapshot 15w32c",
            Self::Snapshot15W33A => "Snapshot 15w33a",
            Self::Snapshot15W33B => "Snapshot 15w33b",
            Self::Snapshot15W33C => "Snapshot 15w33c",
            Self::Snapshot15W34A => "Snapshot 15w34a",
            Self::Snapshot15W34B => "Snapshot 15w34b",
            Self::Snapshot15W34C => "Snapshot 15w34c",
            Self::Snapshot15W34D => "Snapshot 15w34d",
            Self::Snapshot15W35A => "Snapshot 15w35a",
            Self::Snapshot15W35B => "Snapshot 15w35b",
            Self::Snapshot15W35C => "Snapshot 15w35c",
            Self::Snapshot15W35D => "Snapshot 15w35d",
            Self::Snapshot15W35E => "Snapshot 15w35e",
            Self::Snapshot15W36A => "Snapshot 15w36a",
            Self::Snapshot15W36B => "Snapshot 15w36b",
            Self::Snapshot15W36C => "Snapshot 15w36c",
            Self::Snapshot15W36D => "Snapshot 15w36d",
            Self::Snapshot15W37A => "Snapshot 15w37a",
            Self::Snapshot15W38A => "Snapshot 15w38a",
            Self::Snapshot15W38B => "Snapshot 15w38b",
            Self::Snapshot15W39C => "Snapshot 15w39c",
            Self::Snapshot15W40A => "Snapshot 15w40a",
            Self::Snapshot15W40B => "Snapshot 15w40b",
            Self::Snapshot15W41A => "Snapshot 15w41a",
            Self::Snapshot15W41B => "Snapshot 15w41b",
            Self::Snapshot15W42A => "Snapshot 15w42a",
            Self::Snapshot15W43A => "Snapshot 15w43a",
            Self::Snapshot15W43B => "Snapshot 15w43b",
            Self::Snapshot15W43C => "Snapshot 15w43c",
            Self::Snapshot15W44A => "Snapshot 15w44a",
            Self::Snapshot15W44B => "Snapshot 15w44b",
            Self::Snapshot15W45A => "Snapshot 15w45a",
            Self::Snapshot15W46A => "Snapshot 15w46a",
            Self::Snapshot15W47A => "Snapshot 15w47a",
            Self::Snapshot15W47B => "Snapshot 15w47b",
            Self::Snapshot15W47C => "Snapshot 15w47c",
            Self::Snapshot15W49A => "Snapshot 15w49a",
            Self::Snapshot15W49B => "Snapshot 15w49b",
            Self::Snapshot15W50A => "Snapshot 15w50a",
            Self::Snapshot15W51A => "Snapshot 15w51a",
            Self::Snapshot15W51B => "Snapshot 15w51b",
            Self::Snapshot16W02A => "Snapshot 16w02a",
            Self::Snapshot16W03A => "Snapshot 16w03a",
            Self::Snapshot16W04A => "Snapshot 16w04a",
            Self::Snapshot16W05A => "Snapshot 16w05a",
            Self::Snapshot16W05B => "Snapshot 16w05b",
            Self::Snapshot16W06A => "Snapshot 16w06a",
            Self::Snapshot16W07A => "Snapshot 16w07a",
            Self::Snapshot16W07B => "Snapshot 16w07b",
            Self::PreRelease1_9_PRE1 => "Pre Release 1.9/1",
            Self::PreRelease1_9_PRE2 => "Pre Release 1.9/2",
            Self::PreRelease1_9_PRE3 => "Pre Release 1.9/3",
            Self::PreRelease1_9_PRE4 => "Pre Release 1.9/4",
            Self::PreRelease1_9_1_PRE1 => "Pre Release 1.9.1/1",
            Self::Release1_9_1 => "Release 1.9.1",
            Self::PreRelease1_9_3_PRE1 => "Pre Release 1.9.3/1",
            Self::Release1_9_4 => "Release 1.9.4",
            Self::Snapshot16W20A => "Snapshot 16w20a",
            Self::Snapshot16W21A => "Snapshot 16w21a",
            Self::Snapshot16W21B => "Snapshot 16w21b",
            Self::PreRelease1_10_PRE1 => "Pre Release 1.10/1",
            Self::PreRelease1_10_PRE2 => "Pre Release 1.10/2",
            Self::Release1_10_2 => "Release 1.10.2",
            Self::Snapshot16W32A => "Snapshot 16w32a",
            Self::Snapshot16W32B => "Snapshot 16w32b",
            Self::Snapshot16W33A => "Snapshot 16w33a",
            Self::Snapshot16W35A => "Snapshot 16w35a",
            Self::Snapshot16W36A => "Snapshot 16w36a",
            Self::Snapshot16W38A => "Snapshot 16w38a",
            Self::Snapshot16W39A => "Snapshot 16w39a",
            Self::Snapshot16W39B => "Snapshot 16w39b",
            Self::Snapshot16W39C => "Snapshot 16w39c",
            Self::Snapshot16W40A => "Snapshot 16w40a",
            Self::Snapshot16W41A => "Snapshot 16w41a",
            Self::Snapshot16W42A => "Snapshot 16w42a",
            Self::Snapshot16W44A => "Snapshot 16w44a",
            Self::PreRelease1_11_PRE1 => "Pre Release 1.11/1",
            Self::Release1_11 => "Release 1.11",
            Self::Release1_11_2 => "Release 1.11.2",
            Self::Snapshot17W06A => "Snapshot 17w06a",
            Self::Snapshot17W13A => "Snapshot 17w13a",
            Self::Snapshot17W13B => "Snapshot 17w13b",
            Self::Snapshot17W14A => "Snapshot 17w14a",
            Self::Snapshot17W15A => "Snapshot 17w15a",
            Self::Snapshot17W16A => "Snapshot 17w16a",
            Self::Snapshot17W16B => "Snapshot 17w16b",
            Self::Snapshot17W17A => "Snapshot 17w17a",
            Self::Snapshot17W17B => "Snapshot 17w17b",
            Self::Snapshot17W18A => "Snapshot 17w18a",
            Self::Snapshot17W18B => "Snapshot 17w18b",
            Self::PreRelease1_12_PRE1 => "Pre Release 1.12/1",
            Self::PreRelease1_12_PRE2 => "Pre Release 1.12/2",
            Self::PreRelease1_12_PRE3 => "Pre Release 1.12/3",
            Self::PreRelease1_12_PRE4 => "Pre Release 1.12/4",
            Self::PreRelease1_12_PRE5 => "Pre Release 1.12/5",
            Self::PreRelease1_12_PRE6 => "Pre Release 1.12/6",
            Self::PreRelease1_12_PRE7 => "Pre Release 1.12/7",
            Self::Release1_12 => "Release 1.12",
            Self::Snapshot17W31A => "Snapshot 17w31a",
            Self::PreRelease1_12_1_PRE1 => "Pre Release 1.12.1/1",
            Self::Release1_12_1 => "Release 1.12.1",
            Self::PreRelease1_12_2_PRE2 => "Pre Release 1.12.2/2",
            Self::Release1_12_2 => "Release 1.12.2",
            Self::Snapshot17W43A => "Snapshot 17w43a",
            Self::Snapshot17W43B => "Snapshot 17w43b",
            Self::Snapshot17W45A => "Snapshot 17w45a",
            Self::Snapshot17W45B => "Snapshot 17w45b",
            Self::Snapshot17W46A => "Snapshot 17w46a",
            Self::Snapshot17W47A => "Snapshot 17w47a",
            Self::Snapshot17W47B => "Snapshot 17w47b",
            Self::Snapshot17W48A => "Snapshot 17w48a",
            Self::Snapshot17W49A => "Snapshot 17w49a",
            Self::Snapshot17W49B => "Snapshot 17w49b",
            Self::Snapshot17W50A => "Snapshot 17w50a",
            Self::Snapshot18W01A => "Snapshot 18w01a",
            Self::Snapshot18W02A => "Snapshot 18w02a",
            Self::Snapshot18W03A => "Snapshot 18w03a",
            Self::Snapshot18W03B => "Snapshot 18w03b",
            Self::Snapshot18W05A => "Snapshot 18w05a",
            Self::Snapshot18W06A => "Snapshot 18w06a",
            Self::Snapshot18W07A => "Snapshot 18w07a",
            Self::Snapshot18W07B => "Snapshot 18w07b",
            Self::Snapshot18W07C => "Snapshot 18w07c",
            Self::Snapshot18W08A => "Snapshot 18w08a",
            Self::Snapshot18W08B => "Snapshot 18w08b",
            Self::Snapshot18W09A => "Snapshot 18w09a",
            Self::Snapshot18W10A => "Snapshot 18w10a",
            Self::Snapshot18W10B => "Snapshot 18w10b",
            Self::Snapshot18W10C => "Snapshot 18w10c",
            Self::Snapshot18W10D => "Snapshot 18w10d",
            Self::Snapshot18W11A => "Snapshot 18w11a",
            Self::Snapshot18W14A => "Snapshot 18w14a",
            Self::Snapshot18W14B => "Snapshot 18w14b",
            Self::Snapshot18W15A => "Snapshot 18w15a",
            Self::Snapshot18W16A => "Snapshot 18w16a",
            Self::Snapshot18W19A => "Snapshot 18w19a",
            Self::Snapshot18W19B => "Snapshot 18w19b",
            Self::Snapshot18W20A => "Snapshot 18w20a",
            Self::Snapshot18W20B => "Snapshot 18w20b",
            Self::Snapshot18W20C => "Snapshot 18w20c",
            Self::Snapshot18W21A => "Snapshot 18w21a",
            Self::Snapshot18W21B => "Snapshot 18w21b",
            Self::Snapshot18W22A => "Snapshot 18w22a",
            Self::Snapshot18W22B => "Snapshot 18w22b",
            Self::Snapshot18W22C => "Snapshot 18w22c",
            Self::PreRelease1_13_PRE1 => "Pre Release 1.13/1",
            Self::PreRelease1_13_PRE2 => "Pre Release 1.13/2",
            Self::PreRelease1_13_PRE3 => "Pre Release 1.13/3",
            Self::PreRelease1_13_PRE4 => "Pre Release 1.13/4",
            Self::PreRelease1_13_PRE5 => "Pre Release 1.13/5",
            Self::PreRelease1_13_PRE6 => "Pre Release 1.13/6",
            Self::PreRelease1_13_PRE7 => "Pre Release 1.13/7",
            Self::PreRelease1_13_PRE8 => "Pre Release 1.13/8",
            Self::PreRelease1_13_PRE9 => "Pre Release 1.13/9",
            Self::PreRelease1_13_PRE10 => "Pre Release 1.13/10",
            Self::Release1_13 => "Release 1.13",
            Self::Snapshot18W30A => "Snapshot 18w30a",
            Self::Snapshot18W30B => "Snapshot 18w30b",
            Self::Snapshot18W31A => "Snapshot 18w31a",
            Self::Snapshot18W32A => "Snapshot 18w32a",
            Self::Snapshot18W33A => "Snapshot 18w33a",
            Self::PreRelease1_13_1_PRE1 => "Pre Release 1.13.1/1",
            Self::PreRelease1_13_1_PRE2 => "Pre Release 1.13.1/2",
            Self::Release1_13_1 => "Release 1.13.1",
            Self::PreRelease1_13_2_PRE1 => "Pre Release 1.13.2/1",
            Self::PreRelease1_13_2_PRE2 => "Pre Release 1.13.2/2",
            Self::Release1_13_2 => "Release 1.13.2",
            Self::Snapshot18W43A => "Snapshot 18w43a",
            Self::Snapshot18W43B => "Snapshot 18w43b",
            Self::Snapshot18W43C => "Snapshot 18w43c",
            Self::Snapshot18W44A => "Snapshot 18w44a",
            Self::Snapshot18W45A => "Snapshot 18w45a",
            Self::Snapshot18W46A => "Snapshot 18w46a",
            Self::Snapshot18W47A => "Snapshot 18w47a",
            Self::Snapshot18W47B => "Snapshot 18w47b",
            Self::Snapshot18W48A => "Snapshot 18w48a",
            Self::Snapshot18W48B => "Snapshot 18w48b",
            Self::Snapshot18W49A => "Snapshot 18w49a",
            Self::Snapshot18W50A => "Snapshot 18w50a",
            Self::Snapshot19W02A => "Snapshot 19w02a",
            Self::Snapshot19W03A => "Snapshot 19w03a",
            Self::Snapshot19W03B => "Snapshot 19w03b",
            Self::Snapshot19W03C => "Snapshot 19w03c",
            Self::Snapshot19W04A => "Snapshot 19w04a",
            Self::Snapshot19W04B => "Snapshot 19w04b",
            Self::Snapshot19W05A => "Snapshot 19w05a",
            Self::Snapshot19W06A => "Snapshot 19w06a",
            Self::Snapshot19W07A => "Snapshot 19w07a",
            Self::Snapshot19W08A => "Snapshot 19w08a",
            Self::Snapshot19W08B => "Snapshot 19w08b",
            Self::Snapshot19W09A => "Snapshot 19w09a",
            Self::Snapshot19W11A => "Snapshot 19w11a",
            Self::Snapshot19W11B => "Snapshot 19w11b",
            Self::Snapshot19W12A => "Snapshot 19w12a",
            Self::Snapshot19W12B => "Snapshot 19w12b",
            Self::Snapshot19W13A => "Snapshot 19w13a",
            Self::Snapshot19W13B => "Snapshot 19w13b",
            Self::Snapshot19W14A => "Snapshot 19w14a",
            Self::Snapshot19W14B => "Snapshot 19w14b",
            Self::PreRelease1_14_PRE1 => "Pre Release 1.14/1",
            Self::PreRelease1_14_PRE2 => "Pre Release 1.14/2",
            Self::PreRelease1_14_PRE3 => "Pre Release 1.14/3",
            Self::PreRelease1_14_PRE4 => "Pre Release 1.14/4",
            Self::PreRelease1_14_PRE5 => "Pre Release 1.14/5",
            Self::Release1_14 => "Release 1.14",
            Self::PreRelease1_14_1_PRE1 => "Pre Release 1.14.1/1",
            Self::PreRelease1_14_1_PRE2 => "Pre Release 1.14.1/2",
            Self::Release1_14_1 => "Release 1.14.1",
            Self::PreRelease1_14_2_PRE1 => "Pre Release 1.14.2/1",
            Self::PreRelease1_14_2_PRE2 => "Pre Release 1.14.2/2",
            Self::PreRelease1_14_2_PRE3 => "Pre Release 1.14.2/3",
            Self::PreRelease1_14_2_PRE4 => "Pre Release 1.14.2/4",
            Self::Release1_14_2 => "Release 1.14.2",
            Self::PreRelease1_14_3_PRE1 => "Pre Release 1.14.3/1",
            Self::PreRelease1_14_3_PRE2 => "Pre Release 1.14.3/2",
            Self::PreRelease1_14_3_PRE3 => "Pre Release 1.14.3/3",
            Self::PreRelease1_14_3_PRE4 => "Pre Release 1.14.3/4",
            Self::Release1_14_3 => "Release 1.14.3",
            Self::PreRelease1_14_4_PRE1 => "Pre Release 1.14.4/1",
            Self::PreRelease1_14_4_PRE2 => "Pre Release 1.14.4/2",
            Self::PreRelease1_14_4_PRE3 => "Pre Release 1.14.4/3",
            Self::PreRelease1_14_4_PRE4 => "Pre Release 1.14.4/4",
            Self::PreRelease1_14_4_PRE5 => "Pre Release 1.14.4/5",
            Self::PreRelease1_14_4_PRE6 => "Pre Release 1.14.4/6",
            Self::PreRelease1_14_4_PRE7 => "Pre Release 1.14.4/7",
            Self::Release1_14_4 => "Release 1.14.4",
            Self::Snapshot19W34A => "Snapshot 19w34a",
            Self::Snapshot19W35A => "Snapshot 19w35a",
            Self::Snapshot19W36A => "Snapshot 19w36a",
            Self::Snapshot19W37A => "Snapshot 19w37a",
            Self::Snapshot19W38A => "Snapshot 19w38a",
            Self::Snapshot19W38B => "Snapshot 19w38b",
            Self::Snapshot19W39A => "Snapshot 19w39a",
            Self::Snapshot19W40A => "Snapshot 19w40a",
            Self::Snapshot19W41A => "Snapshot 19w41a",
            Self::Snapshot19W42A => "Snapshot 19w42a",
            Self::Snapshot19W44A => "Snapshot 19w44a",
            Self::Snapshot19W45A => "Snapshot 19w45a",
            Self::Snapshot19W45B => "Snapshot 19w45b",
            Self::Snapshot19W46A => "Snapshot 19w46a",
            Self::Snapshot19W46B => "Snapshot 19w46b",
            Self::PreRelease1_15_PRE1 => "Pre Release 1.15/1",
            Self::PreRelease1_15_PRE2 => "Pre Release 1.15/2",
            Self::PreRelease1_15_PRE3 => "Pre Release 1.15/3",
            Self::PreRelease1_15_PRE4 => "Pre Release 1.15/4",
            Self::PreRelease1_15_PRE5 => "Pre Release 1.15/5",
            Self::PreRelease1_15_PRE6 => "Pre Release 1.15/6",
            Self::PreRelease1_15_PRE7 => "Pre Release 1.15/7",
            Self::Release1_15 => "Release 1.15",
            Self::PreRelease1_15_1_PRE1 => "Pre Release 1.15.1/1",
            Self::Release1_15_1 => "Release 1.15.1",
            Self::PreRelease1_15_2_PRE1 => "Pre Release 1.15.2/1",
            Self::PreRelease1_15_2_PRE2 => "Pre Release 1.15.2/2",
            Self::Release1_15_2 => "Release 1.15.2",
            Self::Snapshot20W06A => "Snapshot 20w06a",
            Self::Snapshot20W07A => "Snapshot 20w07a",
            Self::Snapshot20W08A => "Snapshot 20w08a",
            Self::Snapshot20W09A => "Snapshot 20w09a",
            Self::Snapshot20W10A => "Snapshot 20w10a",
            Self::Snapshot20W11A => "Snapshot 20w11a",
            Self::Snapshot20W12A => "Snapshot 20w12a",
            Self::Snapshot20W13A => "Snapshot 20w13a",
            Self::Snapshot20W13B => "Snapshot 20w13b",
            Self::Snapshot20W14A => "Snapshot 20w14a",
            Self::Snapshot20W15A => "Snapshot 20w15a",
            Self::Snapshot20W16A => "Snapshot 20w16a",
            Self::Snapshot20W17A => "Snapshot 20w17a",
            Self::Snapshot20W18A => "Snapshot 20w18a",
            Self::Snapshot20W19A => "Snapshot 20w19a",
            Self::Snapshot20W20A => "Snapshot 20w20a",
            Self::Snapshot20W20B => "Snapshot 20w20b",
            Self::Snapshot20W21A => "Snapshot 20w21a",
            Self::Snapshot20W22A => "Snapshot 20w22a",
            Self::PreRelease1_16_PRE1 => "Pre Release 1.16/1",
            Self::PreRelease1_16_PRE2 => "Pre Release 1.16/2",
            Self::PreRelease1_16_PRE3 => "Pre Release 1.16/3",
            Self::PreRelease1_16_PRE4 => "Pre Release 1.16/4",
            Self::PreRelease1_16_PRE5 => "Pre Release 1.16/5",
            Self::PreRelease1_16_PRE6 => "Pre Release 1.16/6",
            Self::PreRelease1_16_PRE7 => "Pre Release 1.16/7",
            Self::PreRelease1_16_PRE8 => "Pre Release 1.16/8",
            Self::ReleaseCandidate1_16_RC1 => "Release Candidate 1.16/1",
            Self::Release1_16 => "Release 1.16",
            Self::Release1_16_1 => "Release 1.16.1",
            Self::Snapshot20W27A => "Snapshot 20w27a",
            Self::Snapshot20W28A => "Snapshot 20w28a",
            Self::Snapshot20W29A => "Snapshot 20w29a",
            Self::Snapshot20W30A => "Snapshot 20w30a",
            Self::PreRelease1_16_2_PRE1 => "Pre Release 1.16.2/1",
            Self::PreRelease1_16_2_PRE2 => "Pre Release 1.16.2/2",
            Self::PreRelease1_16_2_PRE3 => "Pre Release 1.16.2/3",
            Self::ReleaseCandidate1_16_2_RC1 => "Release Candidate 1.16.2/1",
            Self::ReleaseCandidate1_16_2_RC2 => "Release Candidate 1.16.2/2",
            Self::Release1_16_2 => "Release 1.16.2",
            Self::ReleaseCandidate1_16_3_RC1 => "Release Candidate 1.16.3/1",
            Self::Release1_16_3 => "Release 1.16.3",
            Self::PreRelease1_16_4_PRE1 => "Pre Release 1.16.4/1",
            Self::PreRelease1_16_4_PRE2 => "Pre Release 1.16.4/2",
            Self::ReleaseCandidate1_16_4_RC1 => "Release Candidate 1.16.4/1",
            Self::ReleaseCandidate1_16_5_RC1 => "Release Candidate 1.16.5/1",
            Self::Release1_16_5 => "Release 1.16.5",
            Self::Snapshot20W45A => "Snapshot 20w45a",
            Self::Snapshot20W46A => "Snapshot 20w46a",
            Self::Snapshot20W48A => "Snapshot 20w48a",
            Self::Snapshot20W49A => "Snapshot 20w49a",
            Self::Snapshot20W51A => "Snapshot 20w51a",
            Self::Snapshot21W03A => "Snapshot 21w03a",
            Self::Snapshot21W05A => "Snapshot 21w05a",
            Self::Snapshot21W05B => "Snapshot 21w05b",
            Self::Snapshot21W06A => "Snapshot 21w06a",
            Self::Snapshot21W07A => "Snapshot 21w07a",
            Self::Snapshot21W08A => "Snapshot 21w08a",
            Self::Snapshot21W08B => "Snapshot 21w08b",
            Self::Snapshot21W10A => "Snapshot 21w10a",
            Self::Snapshot21W11A => "Snapshot 21w11a",
            Self::Snapshot21W13A => "Snapshot 21w13a",
            Self::Snapshot21W14A => "Snapshot 21w14a",
            Self::Snapshot21W15A => "Snapshot 21w15a",
            Self::Snapshot21W16A => "Snapshot 21w16a",
            Self::Snapshot21W17A => "Snapshot 21w17a",
            Self::Snapshot21W18A => "Snapshot 21w18a",
            Self::Snapshot21W19A => "Snapshot 21w19a",
            Self::Snapshot21W20A => "Snapshot 21w20a",
            Self::PreRelease1_17_PRE1 => "Pre Release 1.17/1",
            Self::PreRelease1_17_PRE2 => "Pre Release 1.17/2",
            Self::PreRelease1_17_PRE3 => "Pre Release 1.17/3",
            Self::PreRelease1_17_PRE4 => "Pre Release 1.17/4",
            Self::PreRelease1_17_PRE5 => "Pre Release 1.17/5",
            Self::ReleaseCandidate1_17_RC1 => "Release Candidate 1.17/1",
            Self::ReleaseCandidate1_17_RC2 => "Release Candidate 1.17/2",
            Self::Release1_17 => "Release 1.17",
            Self::PreRelease1_17_1_PRE1 => "Pre Release 1.17.1/1",
            Self::PreRelease1_17_1_PRE2 => "Pre Release 1.17.1/2",
            Self::PreRelease1_17_1_PRE3 => "Pre Release 1.17.1/3",
            Self::ReleaseCandidate1_17_1_RC1 => "Release Candidate 1.17.1/1",
            Self::ReleaseCandidate1_17_1_RC2 => "Release Candidate 1.17.1/2",
            Self::Release1_17_1 => "Release 1.17.1",
            Self::Snapshot21W37A => "Snapshot 21w37a",
            Self::Snapshot21W38A => "Snapshot 21w38a",
            Self::Snapshot21W39A => "Snapshot 21w39a",
            Self::Snapshot21W40A => "Snapshot 21w40a",
            Self::Snapshot21W41A => "Snapshot 21w41a",
            Self::Snapshot21W42A => "Snapshot 21w42a",
            Self::Snapshot21W43A => "Snapshot 21w43a",
            Self::Snapshot21W44A => "Snapshot 21w44a",
            Self::PreRelease1_18_PRE1 => "Pre Release 1.18/1",
            Self::PreRelease1_18_PRE2 => "Pre Release 1.18/2",
            Self::PreRelease1_18_PRE3 => "Pre Release 1.18/3",
            Self::PreRelease1_18_PRE4 => "Pre Release 1.18/4",
            Self::PreRelease1_18_PRE5 => "Pre Release 1.18/5",
            Self::PreRelease1_18_PRE6 => "Pre Release 1.18/6",
            Self::PreRelease1_18_PRE7 => "Pre Release 1.18/7",
            Self::PreRelease1_18_PRE8 => "Pre Release 1.18/8",
            Self::ReleaseCandidate1_18_RC1 => "Release Candidate 1.18/1",
            Self::ReleaseCandidate1_18_RC2 => "Release Candidate 1.18/2",
            Self::ReleaseCandidate1_18_RC3 => "Release Candidate 1.18/3",
            Self::ReleaseCandidate1_18_RC4 => "Release Candidate 1.18/4",
            Self::PreRelease1_18_1_PRE1 => "Pre Release 1.18.1/1",
            Self::ReleaseCandidate1_18_1_RC1 => "Release Candidate 1.18.1/1",
            Self::ReleaseCandidate1_18_1_RC2 => "Release Candidate 1.18.1/2",
            Self::ReleaseCandidate1_18_1_RC3 => "Release Candidate 1.18.1/3",
            Self::Release1_18_1 => "Release 1.18.1",
            Self::Snapshot22W03A => "Snapshot 22w03a",
            Self::Snapshot22W05A => "Snapshot 22w05a",
            Self::Snapshot22W06A => "Snapshot 22w06a",
            Self::Snapshot22W07A => "Snapshot 22w07a",
            Self::PreRelease1_18_2_PRE1 => "Pre Release 1.18.2/1",
            Self::PreRelease1_18_2_PRE2 => "Pre Release 1.18.2/2",
            Self::PreRelease1_18_2_PRE3 => "Pre Release 1.18.2/3",
            Self::ReleaseCandidate1_18_2_RC1 => "Release Candidate 1.18.2/1",
            Self::Release1_18_2 => "Release 1.18.2",
            Self::Snapshot22W11A => "Snapshot 22w11a",
            Self::Snapshot22W12A => "Snapshot 22w12a",
            Self::Snapshot22W13A => "Snapshot 22w13a",
            Self::Snapshot22W13ONEBLOCKATATIME => "Snapshot 22w13oneBlockAtATime",
            Self::Snapshot22W14A => "Snapshot 22w14a",
            Self::Snapshot22W15A => "Snapshot 22w15a",
            Self::Snapshot22W16A => "Snapshot 22w16a",
            Self::Snapshot22W16B => "Snapshot 22w16b",
            Self::Snapshot22W17A => "Snapshot 22w17a",
            Self::Snapshot22W18A => "Snapshot 22w18a",
            Self::Snapshot22W19A => "Snapshot 22w19a",
            Self::PreRelease1_19_PRE1 => "Pre Release 1.19/1",
            Self::PreRelease1_19_PRE2 => "Pre Release 1.19/2",
            Self::PreRelease1_19_PRE3 => "Pre Release 1.19/3",
            Self::PreRelease1_19_PRE4 => "Pre Release 1.19/4",
            Self::PreRelease1_19_PRE5 => "Pre Release 1.19/5",
            Self::ReleaseCandidate1_19_RC1 => "Release Candidate 1.19/1",
            Self::ReleaseCandidate1_19_RC2 => "Release Candidate 1.19/2",
            Self::Release1_19 => "Release 1.19",
            Self::Snapshot22W24A => "Snapshot 22w24a",
            Self::PreRelease1_19_1_PRE1 => "Pre Release 1.19.1/1",
            Self::ReleaseCandidate1_19_1_RC1 => "Release Candidate 1.19.1/1",
            Self::PreRelease1_19_1_PRE2 => "Pre Release 1.19.1/2",
            Self::PreRelease1_19_1_PRE3 => "Pre Release 1.19.1/3",
            Self::PreRelease1_19_1_PRE4 => "Pre Release 1.19.1/4",
            Self::PreRelease1_19_1_PRE5 => "Pre Release 1.19.1/5",
            Self::PreRelease1_19_1_PRE6 => "Pre Release 1.19.1/6",
            Self::ReleaseCandidate1_19_1_RC2 => "Release Candidate 1.19.1/2",
            Self::ReleaseCandidate1_19_1_RC3 => "Release Candidate 1.19.1/3",
            Self::ReleaseCandidate1_19_2_RC1 => "Release Candidate 1.19.2/1",
            Self::ReleaseCandidate1_19_2_RC2 => "Release Candidate 1.19.2/2",
            Self::Release1_19_2 => "Release 1.19.2",
            Self::Snapshot22W42A => "Snapshot 22w42a",
            Self::Snapshot22W43A => "Snapshot 22w43a",
            Self::Snapshot22W44A => "Snapshot 22w44a",
            Self::Snapshot22W45A => "Snapshot 22w45a",
            Self::Snapshot22W46A => "Snapshot 22w46a",
            Self::PreRelease1_19_3_PRE1 => "Pre Release 1.19.3/1",
            Self::PreRelease1_19_3_PRE2 => "Pre Release 1.19.3/2",
            Self::PreRelease1_19_3_PRE3 => "Pre Release 1.19.3/3",
            Self::ReleaseCandidate1_19_3_RC1 => "Release Candidate 1.19.3/1",
            Self::ReleaseCandidate1_19_3_RC2 => "Release Candidate 1.19.3/2",
            Self::ReleaseCandidate1_19_3_RC3 => "Release Candidate 1.19.3/3",
            Self::Release1_19_3 => "Release 1.19.3",
            Self::Snapshot23W03A => "Snapshot 23w03a",
            Self::Snapshot23W04A => "Snapshot 23w04a",
            Self::Snapshot23W05A => "Snapshot 23w05a",
            Self::Snapshot23W06A => "Snapshot 23w06a",
            Self::Snapshot23W07A => "Snapshot 23w07a",
            Self::PreRelease1_19_4_PRE1 => "Pre Release 1.19.4/1",
            Self::PreRelease1_19_4_PRE2 => "Pre Release 1.19.4/2",
            Self::PreRelease1_19_4_PRE3 => "Pre Release 1.19.4/3",
            Self::PreRelease1_19_4_PRE4 => "Pre Release 1.19.4/4",
            Self::ReleaseCandidate1_19_4_RC1 => "Release Candidate 1.19.4/1",
            Self::ReleaseCandidate1_19_4_RC2 => "Release Candidate 1.19.4/2",
            Self::ReleaseCandidate1_19_4_RC3 => "Release Candidate 1.19.4/3",
            Self::Release1_19_4 => "Release 1.19.4",
            Self::Snapshot23W12A => "Snapshot 23w12a",
            Self::Snapshot23W13A => "Snapshot 23w13a",
            Self::Snapshot23W14A => "Snapshot 23w14a",
            Self::Snapshot23W16A => "Snapshot 23w16a",
            Self::Snapshot23W17A => "Snapshot 23w17a",
            Self::Snapshot23W18A => "Snapshot 23w18a",
            Self::PreRelease1_20_PRE1 => "Pre Release 1.20/1",
            Self::PreRelease1_20_PRE2 => "Pre Release 1.20/2",
            Self::PreRelease1_20_PRE3 => "Pre Release 1.20/3",
            Self::PreRelease1_20_PRE4 => "Pre Release 1.20/4",
            Self::PreRelease1_20_PRE5 => "Pre Release 1.20/5",
            Self::PreRelease1_20_PRE6 => "Pre Release 1.20/6",
            Self::PreRelease1_20_PRE7 => "Pre Release 1.20/7",
            Self::ReleaseCandidate1_20_RC1 => "Release Candidate 1.20/1",
            Self::ReleaseCandidate1_20_1_RC1 => "Release Candidate 1.20.1/1",
            Self::Release1_20_1 => "Release 1.20.1",
            Self::Snapshot23W31A => "Snapshot 23w31a",
            Self::Snapshot23W32A => "Snapshot 23w32a",
            Self::Snapshot23W33A => "Snapshot 23w33a",
            Self::Snapshot23W35A => "Snapshot 23w35a",
            Self::PreRelease1_20_2_PRE1 => "Pre Release 1.20.2/1",
            Self::PreRelease1_20_2_PRE2 => "Pre Release 1.20.2/2",
            Self::PreRelease1_20_2_PRE3 => "Pre Release 1.20.2/3",
            Self::PreRelease1_20_2_PRE4 => "Pre Release 1.20.2/4",
            Self::ReleaseCandidate1_20_2_RC1 => "Release Candidate 1.20.2/1",
            Self::ReleaseCandidate1_20_2_RC2 => "Release Candidate 1.20.2/2",
            Self::Release1_20_2 => "Release 1.20.2",
            Self::Snapshot23W40A => "Snapshot 23w40a",
            Self::Snapshot23W41A => "Snapshot 23w41a",
            Self::Snapshot23W42A => "Snapshot 23w42a",
            Self::Snapshot23W43A => "Snapshot 23w43a",
            Self::Snapshot23W43B => "Snapshot 23w43b",
            Self::Snapshot23W44A => "Snapshot 23w44a",
            Self::Snapshot23W45A => "Snapshot 23w45a",
            Self::Snapshot23W46A => "Snapshot 23w46a",
            Self::PreRelease1_20_3_PRE1 => "Pre Release 1.20.3/1",
            Self::PreRelease1_20_3_PRE2 => "Pre Release 1.20.3/2",
            Self::PreRelease1_20_3_PRE3 => "Pre Release 1.20.3/3",
            Self::PreRelease1_20_3_PRE4 => "Pre Release 1.20.3/4",
            Self::ReleaseCandidate1_20_3_RC1 => "Release Candidate 1.20.3/1",
            Self::ReleaseCandidate1_20_4_RC1 => "Release Candidate 1.20.4/1",
            Self::Release1_20_4 => "Release 1.20.4",
        }
    }
}
