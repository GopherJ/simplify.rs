use criterion::Criterion;
use simplify::{Point as P, Simplify};

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64
}

impl P for Point {
    fn x(&self) -> f64 {
        self.x 
    }

    fn y(&self) -> f64 {
        self.y
    }
}


fn simplify_bench(c: &mut Criterion) {
     c.bench_function("simplify", move |b| b.iter(|| { 
        let mut points: Vec<Point> = vec![
            Point{x:224.55f64,y:250.15f64},Point{x:224.69f64,y:250.61f64},Point{x:225.15f64,y:249.8f64},Point{x:224.16f64,y:249.25f64},Point{x:224.26f64,y:248.87f64},Point{x:224.93f64,y:247.89f64},Point{x:225.18f64,y:246.9f64},Point{x:226.56f64,y:245.62f64},Point{x:226.9f64,y:245.04f64},Point{x:226.91f64,y:244.19f64},Point{x:227.47f64,y:244.29f64},Point{x:227.98f64,y:244.1f64},Point{x:229.79f64,y:244.14f64},Point{x:231.58f64,y:243.65f64},Point{x:232.32f64,y:242.32f64},Point{x:232.76f64,y:242.1f64},Point{x:233.31f64,y:241.45f64},Point{x:233.59f64,y:240.66f64},Point{x:233.38f64,y:238.66f64},Point{x:234.32f64,y:237.77f64},Point{x:234.98f64,y:236.06f64},Point{x:235.45f64,y:235.85f64},Point{x:237.05f64,y:235.71f64},Point{x:237.44f64,y:235.95f64},Point{x:238.8f64,y:235.98f64},Point{x:240.13f64,y:234.6f64},Point{x:243.53f64,y:233.18f64},Point{x:243.72f64,y:232.87f64},Point{x:244.21f64,y:232.76f64},Point{x:244.67f64,y:232.26f64},Point{x:244.79f64,y:231.8f64},Point{x:245.39f64,y:231.58f64},Point{x:245.71f64,y:231.27f64},Point{x:247.44f64,y:229f64},Point{x:248.09f64,y:228.83f64},Point{x:248.75f64,y:228.18f64},Point{x:248.91f64,y:227.75f64},Point{x:249.91f64,y:226.75f64},Point{x:250.68f64,y:225.45f64},Point{x:251.76f64,y:224.93f64},Point{x:253.59f64,y:223.23f64},Point{x:254.27f64,y:222.87f64},Point{x:254.68f64,y:222.92f64},Point{x:255.45f64,y:222.66f64},Point{x:255.65f64,y:222.25f64},Point{x:256.48f64,y:221.57f64},Point{x:256.9f64,y:221.42f64},Point{x:257.8f64,y:220.52f64},Point{x:258.29f64,y:220.46f64},Point{x:259.12f64,y:219.84f64},Point{x:259.65f64,y:219.81f64},Point{x:259.69f64,y:219.55f64},Point{x:259.94f64,y:219.41f64},Point{x:259.96f64,y:219.1f64},Point{x:260.68f64,y:218.47f64},Point{x:260.62f64,y:218.23f64},Point{x:260.98f64,y:218.1f64},Point{x:261.38f64,y:217.5f64},Point{x:262.15f64,y:216.86f64},Point{x:262.15f64,y:216f64},Point{x:262.59f64,y:215.31f64},Point{x:263f64,y:215.52f64},Point{x:263.31f64,y:215.31f64},Point{x:265.12f64,y:215.33f64},Point{x:265.37f64,y:215.08f64},Point{x:266.19f64,y:214.86f64},Point{x:267.11f64,y:214.04f64},Point{x:267.76f64,y:213.81f64},Point{x:267.97f64,y:212.72f64},Point{x:268.57f64,y:212.22f64},Point{x:269f64,y:212.1f64},Point{x:269.13f64,y:211.31f64},Point{x:270.22f64,y:209.55f64},Point{x:270.42f64,y:208.65f64},Point{x:271.35f64,y:206.48f64},Point{x:271.16f64,y:204.85f64},Point{x:271.25f64,y:204.51f64},Point{x:272.26f64,y:203.54f64},Point{x:272.52f64,y:203.4f64},Point{x:272.91f64,y:203.47f64},Point{x:273.16f64,y:203.05f64},Point{x:273.09f64,y:202.64f64},Point{x:273.57f64,y:201.84f64},Point{x:273.24f64,y:200.81f64},Point{x:273.26f64,y:200.35f64},Point{x:272.56f64,y:198.59f64},Point{x:272.47f64,y:197.87f64},Point{x:272.65f64,y:196.88f64},Point{x:272.18f64,y:195.66f64},Point{x:272.46f64,y:194.59f64},Point{x:272.82f64,y:194.1f64},Point{x:273.12f64,y:192.16f64},Point{x:274.21f64,y:190.81f64},Point{x:276.63f64,y:189.43f64},Point{x:277.2f64,y:189.42f64},Point{x:277.62f64,y:189.03f64},Point{x:278f64,y:188.43f64},Point{x:278.14f64,y:187.72f64},Point{x:278.89f64,y:187.1f64},Point{x:279.35f64,y:184.47f64},Point{x:279.94f64,y:182.84f64},Point{x:280.02f64,y:182.02f64},Point{x:280.36f64,y:181.41f64},Point{x:282.49f64,y:180.2f64},Point{x:283.6f64,y:180.14f64},Point{x:285.89f64,y:177.93f64},Point{x:286.51f64,y:177.74f64},Point{x:286.51f64,y:174.97f64},Point{x:286.88f64,y:173.04f64},Point{x:286.83f64,y:172.31f64},Point{x:288.11f64,y:171.63f64},Point{x:288.83f64,y:170.76f64},Point{x:289.02f64,y:170.83f64},Point{x:289.34f64,y:170.35f64},Point{x:289.37f64,y:169.96f64},Point{x:290.07f64,y:169.32f64},Point{x:290.07f64,y:168.66f64},Point{x:290.41f64,y:168.18f64},Point{x:290.56f64,y:167.19f64},Point{x:291.34f64,y:165.93f64},Point{x:291.34f64,y:164.45f64},Point{x:292.41f64,y:159.37f64},Point{x:293.32f64,y:158.78f64},Point{x:294.89f64,y:157.33f64},Point{x:295.21f64,y:156.86f64},Point{x:295.71f64,y:156.55f64},Point{x:296.26f64,y:156.57f64},Point{x:296.91f64,y:155.64f64},Point{x:298.25f64,y:155.53f64},Point{x:298.69f64,y:155.19f64},Point{x:299.08f64,y:155.3f64},Point{x:299.69f64,y:155.09f64},Point{x:300.4f64,y:155.31f64},Point{x:301.77f64,y:155.4f64},Point{x:302.39f64,y:155.17f64},Point{x:303.28f64,y:155.17f64},Point{x:304.16f64,y:154.83f64},Point{x:305f64,y:154.86f64},Point{x:305.76f64,y:154.33f64},Point{x:306.8f64,y:154.06f64},Point{x:307.25f64,y:153.61f64},Point{x:307.94f64,y:153.43f64},Point{x:308.74f64,y:153.92f64},Point{x:310.09f64,y:153.8f64},Point{x:310.87f64,y:153.12f64},Point{x:311.14f64,y:152.68f64},Point{x:311.1f64,y:152.41f64},Point{x:311.44f64,y:152.21f64},Point{x:312.63f64,y:152.11f64},Point{x:313.39f64,y:151.64f64},Point{x:314.04f64,y:151.61f64},Point{x:314.49f64,y:151.27f64},Point{x:314.95f64,y:151.37f64},Point{x:315.01f64,y:150.76f64},Point{x:315.63f64,y:150.29f64},Point{x:316.54f64,y:149.92f64},Point{x:316.67f64,y:149.58f64},Point{x:317.12f64,y:149.2f64},Point{x:317.26f64,y:148.67f64},Point{x:317.1f64,y:148.35f64},Point{x:317.33f64,y:147.82f64},Point{x:318.7f64,y:146.87f64},Point{x:318.92f64,y:146.51f64},Point{x:319.39f64,y:146.2f64},Point{x:319.75f64,y:145.16f64},Point{x:321f64,y:144.76f64},Point{x:321.37f64,y:144.26f64},Point{x:321.61f64,y:143.55f64},Point{x:324.39f64,y:142.35f64},Point{x:325f64,y:141.75f64},Point{x:325.3f64,y:140.82f64},Point{x:325.7f64,y:140.45f64},Point{x:325.73f64,y:140.23f64},Point{x:326.1f64,y:140.08f64},Point{x:326.27f64,y:139.6f64},Point{x:327.29f64,y:139.43f64},Point{x:328.26f64,y:139.49f64},Point{x:328.77f64,y:139.29f64},Point{x:329.75f64,y:138.04f64},Point{x:330.33f64,y:137.57f64},Point{x:331.77f64,y:137.48f64},Point{x:332.2f64,y:137.78f64},Point{x:333.94f64,y:138.17f64},Point{x:334.22f64,y:138.36f64},Point{x:337.6f64,y:138.61f64},Point{x:338.49f64,y:139.04f64},Point{x:338.91f64,y:139.46f64},Point{x:341.06f64,y:139.74f64},Point{x:341.48f64,y:139.96f64},Point{x:342.92f64,y:139.36f64},Point{x:345.95f64,y:139.2f64},Point{x:346.8f64,y:138.88f64},Point{x:347.37f64,y:138.23f64},Point{x:348.87f64,y:138.35f64},Point{x:349.47f64,y:138.79f64},Point{x:352.86f64,y:138.52f64},Point{x:353.58f64,y:138.61f64},Point{x:354f64,y:138.48f64},Point{x:355.41f64,y:138.6f64},Point{x:356.64f64,y:138.3f64},Point{x:358.69f64,y:138.57f64},Point{x:361.11f64,y:138.35f64},Point{x:362.24f64,y:138.46f64},Point{x:362.92f64,y:138.76f64},Point{x:363.58f64,y:138.53f64},Point{x:364.44f64,y:138.48f64},Point{x:365.39f64,y:138.74f64},Point{x:366.01f64,y:138.47f64},Point{x:367.8f64,y:138.1f64},Point{x:369.98f64,y:137.89f64},Point{x:370.82f64,y:138.25f64},Point{x:372.24f64,y:138.21f64},Point{x:373.71f64,y:138.64f64},Point{x:375.12f64,y:138.53f64},Point{x:376.08f64,y:138.63f64},Point{x:379.45f64,y:139.95f64},Point{x:380.38f64,y:140.19f64},Point{x:381.02f64,y:140.07f64},Point{x:381.86f64,y:140.18f64},Point{x:382.52f64,y:140.4f64},Point{x:383.38f64,y:141.16f64},Point{x:385.1f64,y:142f64},Point{x:387.39f64,y:142.51f64},Point{x:387.67f64,y:142.49f64},Point{x:387.78f64,y:142.12f64},Point{x:389.17f64,y:141.03f64},Point{x:389.39f64,y:140.38f64},Point{x:390.47f64,y:140.07f64},Point{x:390.92f64,y:139.47f64},Point{x:391.14f64,y:139.58f64},Point{x:391.28f64,y:139.39f64},Point{x:393.29f64,y:139.65f64},Point{x:395.17f64,y:139.38f64},Point{x:395.55f64,y:139.48f64},Point{x:395.98f64,y:139.3f64},Point{x:398.28f64,y:139.63f64},Point{x:398.75f64,y:139.84f64},Point{x:400.57f64,y:139.55f64},Point{x:402.4f64,y:139.89f64},Point{x:402.45f64,y:140.15f64},Point{x:403.03f64,y:140.26f64},Point{x:404.11f64,y:140.85f64},Point{x:404.97f64,y:141.02f64},Point{x:405.47f64,y:141.48f64},Point{x:405.79f64,y:141.49f64},Point{x:406.09f64,y:141.23f64},Point{x:406.53f64,y:141.13f64},Point{x:407.24f64,y:141.17f64},Point{x:407.51f64,y:141f64},Point{x:408.37f64,y:141.17f64},Point{x:409.52f64,y:141.14f64},Point{x:410.11f64,y:140.33f64},Point{x:410.76f64,y:140.08f64},Point{x:411.69f64,y:140.42f64},Point{x:414.13f64,y:139.64f64},Point{x:414.47f64,y:139.84f64},Point{x:414.82f64,y:139.75f64},Point{x:414.99f64,y:139.32f64},Point{x:417.7f64,y:137.3f64},Point{x:419.56f64,y:135.5f64},Point{x:421.29f64,y:134.3f64},Point{x:421.9f64,y:133.5f64},Point{x:422.7f64,y:131.95f64},Point{x:423.23f64,y:131.48f64},Point{x:423.38f64,y:130.94f64},Point{x:426.08f64,y:129.26f64},Point{x:427.15f64,y:128.21f64},Point{x:427.72f64,y:127.3f64},Point{x:429.37f64,y:126.9f64},Point{x:429.83f64,y:126.52f64},Point{x:430.08f64,y:125.88f64},Point{x:432.07f64,y:125.45f64},Point{x:433.14f64,y:125.07f64},Point{x:434.13f64,y:124.46f64},Point{x:434.88f64,y:123.79f64},Point{x:435.25f64,y:123f64},Point{x:436.4f64,y:122.42f64},Point{x:437.77f64,y:122.05f64},Point{x:439.1f64,y:120.02f64},Point{x:439.6f64,y:119.74f64},Point{x:440.85f64,y:119.64f64},Point{x:441.91f64,y:119.03f64},Point{x:442.73f64,y:118.95f64},Point{x:445.6f64,y:117.92f64},Point{x:446.05f64,y:117.97f64},Point{x:447.34f64,y:117.65f64},Point{x:447.77f64,y:117.29f64},Point{x:448.22f64,y:117.18f64},Point{x:448.99f64,y:116.53f64},Point{x:451.14f64,y:115.9f64},Point{x:452.8f64,y:115.83f64},Point{x:455.08f64,y:115.26f64},Point{x:456.19f64,y:114.32f64},Point{x:458.08f64,y:114.11f64},Point{x:458.64f64,y:113.86f64},Point{x:460f64,y:112.48f64},Point{x:465.64f64,y:111.28f64},Point{x:466.42f64,y:110.95f64},Point{x:467.42f64,y:111f64},Point{x:468.84f64,y:110.79f64},Point{x:469.75f64,y:110.92f64},Point{x:471.64f64,y:110.14f64},Point{x:474.93f64,y:107.87f64},Point{x:475.8f64,y:107.73f64},Point{x:477.38f64,y:107.87f64},Point{x:478.2f64,y:108.1f64},Point{x:481.19f64,y:108.21f64},Point{x:481.8f64,y:107.84f64},Point{x:483.39f64,y:107.3f64},Point{x:486.51f64,y:106.75f64},Point{x:486.37f64,y:107.53f64},Point{x:486.79f64,y:107.98f64},Point{x:487.66f64,y:108.73f64},Point{x:488.86f64,y:109.07f64},Point{x:489.2f64,y:109.45f64},Point{x:491.39f64,y:109.36f64},Point{x:492.01f64,y:109.1f64},Point{x:493.15f64,y:109.03f64},Point{x:493.57f64,y:108.63f64},Point{x:493.79f64,y:108.63f64},Point{x:494.48f64,y:109.16f64},Point{x:495.27f64,y:110.44f64},Point{x:495.78f64,y:110.65f64},Point{x:496.09f64,y:111.22f64},Point{x:497.28f64,y:111.75f64},Point{x:497.3f64,y:112.24f64},Point{x:497.55f64,y:112.68f64},Point{x:498.84f64,y:113.96f64},Point{x:501.08f64,y:115.96f64},Point{x:502.08f64,y:116.42f64},Point{x:502.63f64,y:117.28f64},Point{x:503.71f64,y:118.01f64},Point{x:504.16f64,y:118.77f64},Point{x:504.18f64,y:119.09f64},Point{x:504.74f64,y:119.66f64},Point{x:505.18f64,y:119.8f64},Point{x:507.32f64,y:119.66f64},Point{x:508.21f64,y:119.83f64},Point{x:508.79f64,y:120.46f64},Point{x:509.6f64,y:120.56f64},Point{x:510.02f64,y:121.08f64},Point{x:512.96f64,y:122.35f64},Point{x:513.27f64,y:121.99f64},Point{x:513.66f64,y:122.18f64},Point{x:514.28f64,y:122.2f64},Point{x:514.8f64,y:121.6f64},Point{x:515.49f64,y:121.39f64},Point{x:516.2f64,y:121.4f64},Point{x:517.51f64,y:120.87f64},Point{x:517.86f64,y:121.04f64},Point{x:518.63f64,y:120.89f64},Point{x:518.88f64,y:121.42f64},Point{x:519.95f64,y:122.03f64},Point{x:520.43f64,y:122.49f64},Point{x:520.56f64,y:122.87f64},Point{x:521.93f64,y:123.43f64},Point{x:523.01f64,y:125.73f64},Point{x:524.09f64,y:126.88f64},Point{x:524.55f64,y:126.97f64},Point{x:524.92f64,y:127.38f64},Point{x:525.36f64,y:127.41f64},Point{x:526.63f64,y:127.94f64},Point{x:527.99f64,y:127.95f64},Point{x:528.98f64,y:127.61f64},Point{x:529.57f64,y:127.86f64},Point{x:529.89f64,y:128.14f64},Point{x:530.53f64,y:129.23f64},Point{x:530.87f64,y:129.47f64},Point{x:530.69f64,y:129.79f64},Point{x:530.81f64,y:132.77f64},Point{x:532.48f64,y:134.47f64},Point{x:533.15f64,y:136.66f64},Point{x:533.9f64,y:137.9f64},Point{x:534.03f64,y:138.86f64},Point{x:533.91f64,y:139.82f64},Point{x:534.21f64,y:140.93f64},Point{x:534.77f64,y:141.58f64},Point{x:535.23f64,y:142.66f64},Point{x:536.2f64,y:143.65f64},Point{x:536.69f64,y:144.76f64},Point{x:537.01f64,y:144.47f64},Point{x:537.85f64,y:144.57f64},Point{x:538.5f64,y:144.83f64},Point{x:539.24f64,y:145.53f64},Point{x:539.45f64,y:146.73f64},Point{x:539.27f64,y:147.24f64},Point{x:541.59f64,y:148.01f64},Point{x:542.6f64,y:148.02f64},Point{x:543.15f64,y:148.44f64},Point{x:543.68f64,y:148.54f64},Point{x:546.12f64,y:148.44f64},Point{x:546.26f64,y:148.13f64},Point{x:547.74f64,y:147.7f64},Point{x:548.95f64,y:147.72f64},Point{x:549.68f64,y:147.45f64},Point{x:550.4f64,y:147.56f64},Point{x:550.66f64,y:147.27f64},Point{x:552f64,y:147.31f64},Point{x:552.94f64,y:148.22f64},Point{x:554.75f64,y:148.14f64},Point{x:555.02f64,y:148.64f64},Point{x:555.32f64,y:148.74f64},Point{x:556.16f64,y:148.56f64},Point{x:557.88f64,y:148.83f64},Point{x:559.07f64,y:148.41f64},Point{x:559.7f64,y:148.48f64},Point{x:559.99f64,y:148.85f64},Point{x:560.62f64,y:148.99f64},Point{x:560.92f64,y:149.29f64},Point{x:562.06f64,y:149.76f64},Point{x:565.3f64,y:149.57f64},Point{x:566.51f64,y:148.91f64},Point{x:567.69f64,y:148.91f64},Point{x:567.89f64,y:148.98f64},Point{x:567.77f64,y:149.19f64},Point{x:568.1f64,y:149.46f64},Point{x:568.52f64,y:149.57f64},Point{x:568.94f64,y:149.45f64},Point{x:569.23f64,y:150.05f64},Point{x:570.38f64,y:151.12f64},Point{x:571.1f64,y:152.6f64},Point{x:571.76f64,y:153.09f64},Point{x:575.25f64,y:157.26f64},Point{x:576.85f64,y:157.4f64},Point{x:580.62f64,y:158.15f64},Point{x:581.86f64,y:158.14f64},Point{x:583.83f64,y:157.79f64},Point{x:584.98f64,y:158.17f64},Point{x:586.67f64,y:158.32f64},Point{x:592.8f64,y:158.21f64},Point{x:593.19f64,y:157.95f64},Point{x:595.17f64,y:157.66f64},Point{x:596.17f64,y:157.78f64},Point{x:598.02f64,y:157.63f64},Point{x:598.61f64,y:157.4f64},Point{x:599.48f64,y:157.39f64},Point{x:601.06f64,y:157.06f64},Point{x:601.53f64,y:156.85f64},Point{x:601.68f64,y:157.03f64},Point{x:605.03f64,y:157.05f64},Point{x:606.47f64,y:157.69f64},Point{x:606.9f64,y:157.71f64},Point{x:608.23f64,y:157.27f64},Point{x:608.5f64,y:157.46f64},Point{x:608.81f64,y:157.35f64},Point{x:609.47f64,y:158.05f64},Point{x:609.86f64,y:157.92f64},Point{x:611.26f64,y:158.55f64},Point{x:612.3f64,y:158.44f64},Point{x:614.73f64,y:158.8f64},Point{x:617.74f64,y:159.86f64},Point{x:618.14f64,y:160.29f64},Point{x:617.91f64,y:160.98f64},Point{x:619.71f64,y:164.86f64},Point{x:621.52f64,y:166.36f64},Point{x:622f64,y:167.04f64},Point{x:622.11f64,y:170.69f64},Point{x:622.01f64,y:171.26f64},Point{x:623.66f64,y:173.26f64},Point{x:624.15f64,y:176.75f64},Point{x:624.68f64,y:177.5f64},Point{x:625.05f64,y:179.03f64},Point{x:625.69f64,y:179.83f64},Point{x:626.06f64,y:181.1f64},Point{x:625.92f64,y:185.63f64},Point{x:626.45f64,y:185.96f64},Point{x:626.72f64,y:186.8f64},Point{x:627.07f64,y:186.84f64},Point{x:627.62f64,y:188.65f64},Point{x:628.11f64,y:189.21f64},Point{x:628.09f64,y:190.19f64},Point{x:629.18f64,y:192.37f64},Point{x:629.55f64,y:194.6f64},Point{x:629.93f64,y:194.86f64},Point{x:630.78f64,y:194.7f64},Point{x:632.12f64,y:194.86f64},Point{x:633.06f64,y:195.47f64},Point{x:635.78f64,y:196.31f64},Point{x:638.39f64,y:195.52f64},Point{x:638.9f64,y:195.61f64},Point{x:640.48f64,y:197.63f64},Point{x:640.44f64,y:199.01f64},Point{x:641.26f64,y:200.81f64},Point{x:644.54f64,y:202.78f64},Point{x:645.02f64,y:202.82f64},Point{x:645.55f64,y:202.57f64},Point{x:647.72f64,y:202.67f64},Point{x:648.57f64,y:203.55f64},Point{x:648.85f64,y:203.38f64},Point{x:648.94f64,y:203.12f64},Point{x:649.85f64,y:203.32f64},Point{x:651f64,y:204.22f64},Point{x:651.77f64,y:204.56f64},Point{x:653.14f64,y:206.28f64},Point{x:655.45f64,y:208.18f64},Point{x:656.46f64,y:208.26f64},Point{x:657.19f64,y:208.81f64},Point{x:657.89f64,y:209.06f64},Point{x:658.59f64,y:210.25f64},Point{x:659.45f64,y:210.83f64},Point{x:661.57f64,y:213.37f64},Point{x:664.15f64,y:215.09f64},Point{x:664.54f64,y:215f64},Point{x:664.76f64,y:215.19f64},Point{x:664.73f64,y:215.38f64},Point{x:665.8f64,y:215.98f64},Point{x:666.58f64,y:216.94f64},Point{x:667.21f64,y:217.45f64},Point{x:669.67f64,y:219.1f64},Point{x:669.72f64,y:219.89f64},Point{x:670.07f64,y:220.84f64},Point{x:670.36f64,y:221.01f64},Point{x:671.19f64,y:222.26f64},Point{x:671.55f64,y:222.55f64},Point{x:671.9f64,y:222.07f64},Point{x:672.3f64,y:222.11f64},Point{x:672.9f64,y:221.69f64},Point{x:673.65f64,y:221.62f64},Point{x:674.39f64,y:221.09f64},Point{x:675.68f64,y:220.97f64},Point{x:677.51f64,y:221.33f64},Point{x:677.85f64,y:220.89f64},Point{x:678.6f64,y:220.39f64},Point{x:679.03f64,y:219.79f64},Point{x:681.31f64,y:218.71f64},Point{x:681.6f64,y:217.98f64},Point{x:681.8f64,y:217.85f64},Point{x:682.62f64,y:218.16f64},Point{x:683.68f64,y:217.45f64},Point{x:686.87f64,y:217.67f64},Point{x:687.49f64,y:217.59f64},Point{x:688.81f64,y:218.15f64},Point{x:690.62f64,y:218.21f64},Point{x:695.25f64,y:219.15f64},Point{x:696.58f64,y:218.76f64},Point{x:697.68f64,y:218.83f64},Point{x:699.34f64,y:218.53f64},Point{x:699.44f64,y:218.09f64},Point{x:700.64f64,y:217.98f64},Point{x:701.01f64,y:217.32f64},Point{x:700.98f64,y:216.37f64},Point{x:701.16f64,y:215.87f64},Point{x:702.86f64,y:214.71f64},Point{x:703.12f64,y:214.36f64},Point{x:707.1f64,y:215.09f64},Point{x:707.62f64,y:214.92f64},Point{x:709.34f64,y:215.08f64},Point{x:709.97f64,y:215.36f64},Point{x:710.42f64,y:215.78f64},Point{x:712.26f64,y:215.87f64},Point{x:713.11f64,y:215.63f64},Point{x:713.67f64,y:215.25f64},Point{x:714.04f64,y:215.23f64},Point{x:714.79f64,y:214.47f64},Point{x:715.81f64,y:214.36f64},Point{x:717.26f64,y:213.55f64},Point{x:717.92f64,y:213.57f64},Point{x:718.67f64,y:213.28f64},Point{x:721.25f64,y:212.98f64},Point{x:721.49f64,y:212.81f64},Point{x:721.76f64,y:213.15f64},Point{x:723.17f64,y:213.29f64},Point{x:723.44f64,y:214f64},Point{x:724.02f64,y:214.27f64},Point{x:726.71f64,y:213.43f64},Point{x:726.88f64,y:213.52f64},Point{x:727.13f64,y:213.32f64},Point{x:727.28f64,y:213.43f64},Point{x:727.81f64,y:213.36f64},Point{x:728.05f64,y:213.1f64},Point{x:728.07f64,y:211.23f64},Point{x:728.78f64,y:210.03f64},Point{x:729.98f64,y:208.73f64},Point{x:730.99f64,y:208.38f64},Point{x:732.01f64,y:208.25f64},Point{x:733.02f64,y:207.73f64},Point{x:733.8f64,y:208.35f64},Point{x:735.32f64,y:208.2f64},Point{x:735.9f64,y:207.87f64},Point{x:737.28f64,y:206.1f64},Point{x:737.56f64,y:205.95f64},Point{x:737.78f64,y:206.1f64},Point{x:737.69f64,y:205.78f64},Point{x:738.66f64,y:205.44f64},Point{x:738.96f64,y:204.97f64},Point{x:739.94f64,y:204.77f64},Point{x:742.94f64,y:204.7f64},Point{x:743.39f64,y:204.86f64},Point{x:744.12f64,y:205.44f64},Point{x:745.4f64,y:205.53f64},Point{x:746.71f64,y:205.36f64},Point{x:747.72f64,y:204.96f64},Point{x:747.93f64,y:204.54f64},Point{x:748.7f64,y:204.81f64},Point{x:749.82f64,y:204.91f64},Point{x:750.6f64,y:205.38f64},Point{x:751.17f64,y:205.32f64},Point{x:751.64f64,y:205.58f64},Point{x:751.78f64,y:205.93f64},Point{x:753.07f64,y:206.02f64},Point{x:754.04f64,y:206.39f64},Point{x:754.36f64,y:206.67f64},Point{x:755.78f64,y:206.77f64},Point{x:756.67f64,y:206.5f64},Point{x:756.99f64,y:206.68f64},Point{x:758.49f64,y:206.49f64},Point{x:759.99f64,y:206.84f64},Point{x:760.57f64,y:206.82f64},Point{x:761.03f64,y:207.05f64},Point{x:762.69f64,y:207.01f64},Point{x:763.05f64,y:207.31f64},Point{x:763.56f64,y:207.32f64},Point{x:764.71f64,y:207.68f64},Point{x:765.3f64,y:207.74f64},Point{x:765.97f64,y:207.51f64},Point{x:767.35f64,y:208.01f64},Point{x:767.71f64,y:207.96f64},Point{x:769.98f64,y:208.42f64},Point{x:771.92f64,y:209.93f64},Point{x:773.43f64,y:210.6f64},Point{x:773.77f64,y:211.31f64},Point{x:773.63f64,y:211.74f64},Point{x:773.94f64,y:212.21f64},Point{x:774.16f64,y:212.25f64},Point{x:774.26f64,y:212.59f64},Point{x:775.22f64,y:212.7f64},Point{x:776.36f64,y:213.25f64},Point{x:777.16f64,y:214.24f64},Point{x:777.95f64,y:214.82f64},Point{x:778.4f64,y:214.87f64},Point{x:778.64f64,y:215.14f64},Point{x:778.66f64,y:215.48f64},Point{x:779.21f64,y:215.85f64},Point{x:779.6f64,y:216.87f64},Point{x:780.98f64,y:217.05f64},Point{x:781.53f64,y:217.41f64},Point{x:782.83f64,y:217.49f64},Point{x:784.2f64,y:218.16f64},Point{x:785.44f64,y:218.23f64},Point{x:785.78f64,y:218.02f64},Point{x:787.96f64,y:218.51f64},Point{x:789.45f64,y:218.36f64},Point{x:789.93f64,y:217.8f64},Point{x:792.07f64,y:217.27f64},Point{x:793.31f64,y:216.27f64},Point{x:794.3f64,y:215.97f64},Point{x:796.24f64,y:214.79f64},Point{x:797.09f64,y:214.55f64},Point{x:797.67f64,y:214.73f64},Point{x:798.01f64,y:214.61f64},Point{x:798.25f64,y:214.7f64},Point{x:799.15f64,y:214.29f64},Point{x:800.24f64,y:214.62f64},Point{x:800.4f64,y:215.1f64},Point{x:801.04f64,y:215.54f64},Point{x:802.54f64,y:215.57f64},Point{x:803.32f64,y:216.22f64},Point{x:808.09f64,y:218.4f64},Point{x:808.83f64,y:218.91f64},Point{x:809.39f64,y:218.95f64},Point{x:810.53f64,y:219.73f64},Point{x:811.64f64,y:221f64},Point{x:811.65f64,y:222.65f64},Point{x:813.64f64,y:224.71f64},Point{x:815.18f64,y:225.97f64},Point{x:815.43f64,y:225.97f64},Point{x:816.14f64,y:226.61f64},Point{x:816.68f64,y:226.79f64},Point{x:817.06f64,y:226.68f64},Point{x:817.19f64,y:226.82f64},Point{x:817.85f64,y:228.24f64},Point{x:818.19f64,y:228.64f64},Point{x:818.6f64,y:231.56f64},Point{x:819.14f64,y:232.38f64},Point{x:819.1f64,y:232.81f64},Point{x:819.87f64,y:233.97f64},Point{x:820.77f64,y:236.17f64},Point{x:821.54f64,y:236.39f64},Point{x:823.92f64,y:236.03f64},Point{x:827.23f64,y:236.16f64},Point{x:827.53f64,y:236.42f64},Point{x:828.36f64,y:238.42f64},Point{x:829.89f64,y:239.89f64},Point{x:832.6f64,y:241.26f64},Point{x:834.54f64,y:241.96f64},Point{x:835.07f64,y:241.88f64},Point{x:835.18f64,y:242.14f64},Point{x:835.58f64,y:242.13f64},Point{x:836.17f64,y:242.51f64},Point{x:836.78f64,y:242.66f64},Point{x:837.53f64,y:243.4f64},Point{x:838.37f64,y:243.88f64},Point{x:838.43f64,y:243.5f64},Point{x:838.66f64,y:243.57f64},Point{x:839.04f64,y:244.12f64},Point{x:839.22f64,y:244.92f64},Point{x:841.21f64,y:245.84f64},Point{x:841.83f64,y:246.04f64},Point{x:842.04f64,y:245.77f64},Point{x:842.41f64,y:245.75f64},Point{x:843.26f64,y:245.27f64},Point{x:844.24f64,y:245.57f64},Point{x:845.03f64,y:246.39f64},Point{x:845.85f64,y:246.6f64},Point{x:847f64,y:247.64f64},Point{x:847.34f64,y:247.73f64},Point{x:847.41f64,y:248.18f64},Point{x:848.42f64,y:248.13f64},Point{x:849.7f64,y:248.92f64},Point{x:851f64,y:248.94f64},Point{x:851.44f64,y:249.29f64},Point{x:852.08f64,y:250.28f64},Point{x:852.61f64,y:250.7f64},Point{x:852.83f64,y:251.42f64},Point{x:854.06f64,y:252.12f64},Point{x:854.8f64,y:252.81f64},Point{x:856.59f64,y:253.4f64},Point{x:858.68f64,y:254.64f64},Point{x:859.5f64,y:255.39f64},Point{x:859.88f64,y:255.49f64},Point{x:860.33f64,y:256.75f64},Point{x:861.17f64,y:257.75f64},Point{x:861.43f64,y:258.74f64},Point{x:861.26f64,y:259.2f64},Point{x:861.57f64,y:259.34f64},Point{x:860.95f64,y:260.82f64},Point{x:862.67f64,y:262.25f64},Point{x:862.85f64,y:262.88f64},Point{x:863.1f64,y:263.15f64},Point{x:863.12f64,y:263.53f64},Point{x:864.01f64,y:265f64},Point{x:864.49f64,y:267.98f64},Point{x:864.76f64,y:268.06f64},Point{x:865.21f64,y:268.53f64},Point{x:865.02f64,y:269.11f64},Point{x:864.37f64,y:268.85f64},Point{x:864.15f64,y:269.33f64},Point{x:863.73f64,y:269.69f64},Point{x:863.78f64,y:269.93f64},Point{x:863.88f64,y:269.85f64},Point{x:864.53f64,y:270.31f64},Point{x:864.3f64,y:270.94f64},Point{x:864.43f64,y:271.25f64},Point{x:864.29f64,y:271.5f64},Point{x:863.59f64,y:271.56f64},Point{x:863.07f64,y:272.03f64},Point{x:863.14f64,y:272.21f64},Point{x:861.6f64,y:273.31f64},Point{x:861.47f64,y:273.58f64},Point{x:860.76f64,y:273.83f64},Point{x:860.64f64,y:274.09f64},Point{x:860.96f64,y:274.52f64},Point{x:860.93f64,y:274.95f64},Point{x:861.15f64,y:275.41f64},Point{x:861.42f64,y:275.58f64},Point{x:861.54f64,y:276.09f64},Point{x:861.18f64,y:276.87f64},Point{x:860.91f64,y:276.89f64},Point{x:861.03f64,y:277.41f64},Point{x:860.24f64,y:277.5f64},Point{x:859.28f64,y:278.03f64},Point{x:858.98f64,y:278.5f64},Point{x:858.91f64,y:279.15f64},Point{x:858.01f64,y:279.83f64},Point{x:857.95f64,y:280.3f64},Point{x:858.02f64,y:280.95f64},Point{x:858.56f64,y:281.61f64},Point{x:859.44f64,y:281.91f64},Point{x:859.85f64,y:282.31f64},Point{x:860.42f64,y:282.4f64},Point{x:860.41f64,y:282.97f64},Point{x:861.29f64,y:283.62f64},Point{x:861.73f64,y:284.46f64},Point{x:861.78f64,y:285.25f64},Point{x:861.6f64,y:285.89f64},Point{x:862.09f64,y:286.16f64},Point{x:862.38f64,y:285.9f64},Point{x:862.55f64,y:285.95f64},Point{x:862.9f64,y:286.61f64},Point{x:863.29f64,y:288.07f64},Point{x:863.36f64,y:288.97f64},Point{x:863.98f64,y:289.4f64},Point{x:863.94f64,y:290.43f64},Point{x:864.45f64,y:290.42f64},Point{x:864.57f64,y:290.73f64},Point{x:864.91f64,y:290.59f64},Point{x:865.21f64,y:291.29f64},Point{x:865.48f64,y:291.45f64},Point{x:865.32f64,y:292.52f64},Point{x:865.63f64,y:293.4f64},Point{x:866.1f64,y:294.07f64},Point{x:866.06f64,y:294.62f64},Point{x:866.32f64,y:295.2f64},Point{x:866.04f64,y:296.21f64},Point{x:865.82f64,y:296.36f64},Point{x:865.78f64,y:297.21f64},Point{x:866.12f64,y:298.04f64},Point{x:866.61f64,y:298.23f64},Point{x:866.81f64,y:298.66f64},Point{x:865.76f64,y:299.92f64},Point{x:865.06f64,y:300.33f64},Point{x:864.58f64,y:301.43f64},Point{x:864.79f64,y:302f64},Point{x:864.68f64,y:302.71f64},Point{x:865.32f64,y:303.37f64},Point{x:865.35f64,y:303.62f64},Point{x:865.46f64,y:303.68f64},Point{x:865.63f64,y:303.49f64},Point{x:866.81f64,y:304.67f64},Point{x:867.29f64,y:304.96f64},Point{x:867.33f64,y:305.89f64},Point{x:867.79f64,y:306.17f64},Point{x:866.43f64,y:308.08f64},Point{x:865.7f64,y:308.15f64},Point{x:864.81f64,y:308.75f64},Point{x:864.24f64,y:308.72f64},Point{x:863.18f64,y:308.96f64},Point{x:862.52f64,y:309.54f64},Point{x:861.87f64,y:309.86f64},Point{x:861.49f64,y:310.42f64},Point{x:861.54f64,y:310.76f64},Point{x:861.2f64,y:311.37f64},Point{x:860.66f64,y:311.38f64},Point{x:860.43f64,y:311.04f64},Point{x:859.87f64,y:311.37f64},Point{x:859.84f64,y:311.72f64},Point{x:859.7f64,y:311.8f64},Point{x:859.89f64,y:312.99f64},Point{x:859.71f64,y:313.68f64},Point{x:860.08f64,y:314.35f64},Point{x:859.84f64,y:314.22f64},Point{x:859.59f64,y:314.42f64},Point{x:859.41f64,y:314.31f64},Point{x:859.35f64,y:314.56f64},Point{x:859.05f64,y:314.76f64},Point{x:858.87f64,y:314.68f64},Point{x:858.29f64,y:314.94f64},Point{x:858.49f64,y:315.35f64},Point{x:858.19f64,y:315.6f64},Point{x:858.2f64,y:316.09f64},Point{x:857.97f64,y:316.42f64},Point{x:857.99f64,y:317.18f64},Point{x:857.71f64,y:317.99f64},Point{x:857.81f64,y:318.45f64},Point{x:857.63f64,y:318.77f64},Point{x:857.65f64,y:319.12f64},Point{x:857.32f64,y:319.49f64},Point{x:857.37f64,y:320.92f64},Point{x:857.23f64,y:321.32f64},Point{x:857.23f64,y:322.66f64},Point{x:857.44f64,y:324.03f64},Point{x:857.34f64,y:326.69f64},Point{x:858.08f64,y:327.3f64},Point{x:858.1f64,y:327.6f64},Point{x:857.59f64,y:328.6f64},Point{x:857.21f64,y:328.98f64},Point{x:856.81f64,y:330.34f64},Point{x:856.37f64,y:330.51f64},Point{x:855.98f64,y:330.93f64},Point{x:855.95f64,y:331.89f64},Point{x:855.36f64,y:332.13f64},Point{x:855.38f64,y:332.51f64},Point{x:855.06f64,y:332.61f64},Point{x:854.91f64,y:333.44f64},Point{x:855.21f64,y:334.74f64},Point{x:854.57f64,y:335.21f64},Point{x:854.54f64,y:335.4f64},Point{x:855.01f64,y:336.06f64},Point{x:855f64,y:336.45f64},Point{x:855.28f64,y:336.79f64},Point{x:855.8f64,y:338.6f64},Point{x:856.14f64,y:338.6f64},Point{x:856.23f64,y:338.86f64},Point{x:856.35f64,y:338.63f64},Point{x:856.71f64,y:338.94f64},Point{x:857.05f64,y:338.89f64},Point{x:857.23f64,y:338.64f64},Point{x:858.07f64,y:338.79f64},Point{x:858.2f64,y:338.94f64},Point{x:858.92f64,y:338.57f64},Point{x:859.11f64,y:338.89f64},Point{x:858.81f64,y:339.55f64},Point{x:858.92f64,y:340.41f64},Point{x:859.53f64,y:340.69f64},Point{x:859.84f64,y:341.04f64},Point{x:860.07f64,y:341.37f64},Point{x:860.06f64,y:341.83f64},Point{x:860.66f64,y:342.03f64},Point{x:860.92f64,y:343f64},Point{x:860.55f64,y:343.73f64},Point{x:859.97f64,y:344.14f64},Point{x:859.58f64,y:344.7f64},Point{x:858.97f64,y:346.97f64},Point{x:858.43f64,y:347.23f64},Point{x:857.95f64,y:347.79f64},Point{x:857.37f64,y:348.06f64},Point{x:857.06f64,y:348.95f64},Point{x:856.76f64,y:349.12f64},Point{x:856.84f64,y:349.26f64},Point{x:856.38f64,y:349.83f64},Point{x:856.43f64,y:350.15f64},Point{x:856.12f64,y:350.24f64},Point{x:856f64,y:350.57f64},Point{x:855.55f64,y:350.42f64},Point{x:855.29f64,y:350.5f64},Point{x:854.9f64,y:351.05f64},Point{x:854.52f64,y:351.3f64},Point{x:854.44f64,y:351.92f64},Point{x:854.11f64,y:351.83f64},Point{x:853.89f64,y:352.24f64},Point{x:853.66f64,y:352.25f64},Point{x:853.37f64,y:352.65f64},Point{x:852.5f64,y:352.57f64},Point{x:852.42f64,y:353.03f64},Point{x:852.19f64,y:352.87f64},Point{x:851.42f64,y:352.96f64},Point{x:851.41f64,y:353.57f64},Point{x:850.85f64,y:354.08f64},Point{x:850.76f64,y:354.94f64},Point{x:850.88f64,y:355.32f64},Point{x:850.21f64,y:356.07f64},Point{x:850.17f64,y:356.49f64},Point{x:850.62f64,y:356.9f64},Point{x:851.06f64,y:357.75f64},Point{x:850.59f64,y:357.96f64},Point{x:850.56f64,y:358.47f64},Point{x:849.95f64,y:359.08f64},Point{x:849.84f64,y:359.59f64},Point{x:850.11f64,y:360.29f64},Point{x:849.92f64,y:360.43f64},Point{x:850.33f64,y:360.49f64},Point{x:851.07f64,y:361.18f64},Point{x:852.05f64,y:361.17f64},Point{x:852.13f64,y:361.59f64},Point{x:852.71f64,y:362.1f64},Point{x:852.73f64,y:362.8f64},Point{x:853.19f64,y:363f64},Point{x:853.47f64,y:363.67f64},Point{x:853.87f64,y:363.76f64},Point{x:853.9f64,y:364.28f64},Point{x:854.42f64,y:364.29f64},Point{x:854.65f64,y:365.14f64},Point{x:854.56f64,y:365.53f64},Point{x:854.16f64,y:366.23f64},Point{x:854.24f64,y:366.42f64},Point{x:853.84f64,y:366.47f64},Point{x:852.39f64,y:367.52f64},Point{x:851.57f64,y:368.38f64},Point{x:850.93f64,y:368.68f64},Point{x:851.06f64,y:369f64},Point{x:850.93f64,y:369.23f64},Point{x:850.46f64,y:369.51f64},Point{x:849.74f64,y:370.38f64},Point{x:849.06f64,y:370.71f64},Point{x:848.5f64,y:370.69f64},Point{x:848.23f64,y:370.49f64},Point{x:847.4f64,y:371.33f64},Point{x:847.14f64,y:371.16f64},Point{x:846.94f64,y:370.7f64},Point{x:846.18f64,y:371.02f64},Point{x:845.71f64,y:371.5f64},Point{x:845.04f64,y:371.11f64},Point{x:844.09f64,y:371.89f64},Point{x:844.19f64,y:372.58f64},Point{x:844.02f64,y:373.35f64},Point{x:843.52f64,y:373.95f64},Point{x:843.27f64,y:374.55f64},Point{x:843.51f64,y:376.21f64},Point{x:843.82f64,y:376.83f64},Point{x:843.72f64,y:377.01f64},Point{x:843.96f64,y:377.62f64},Point{x:843.94f64,y:378.01f64},Point{x:844.01f64,y:378.16f64},Point{x:844.38f64,y:378.03f64},Point{x:844.52f64,y:378.57f64},Point{x:844.31f64,y:378.85f64},Point{x:844.75f64,y:380.44f64},Point{x:843.74f64,y:382.15f64},Point{x:843.46f64,y:382.35f64},Point{x:842.91f64,y:382.37f64},Point{x:841.52f64,y:383.67f64},Point{x:841.12f64,y:385.66f64},Point{x:840.87f64,y:385.75f64},Point{x:840.51f64,y:386.39f64},Point{x:840.63f64,y:386.95f64},Point{x:840.12f64,y:388.01f64},Point{x:839.9f64,y:389.41f64},Point{x:839.99f64,y:389.78f64},Point{x:839.57f64,y:390.4f64},Point{x:839.94f64,y:390.99f64},Point{x:839.86f64,y:391.33f64},Point{x:839.99f64,y:391.65f64},Point{x:839.87f64,y:391.82f64},Point{x:840.73f64,y:392.78f64},Point{x:841.69f64,y:393.3f64},Point{x:842.18f64,y:394.49f64},Point{x:842.93f64,y:394.88f64},Point{x:843.07f64,y:395.1f64},Point{x:843.39f64,y:396.4f64},Point{x:844.27f64,y:397.32f64},Point{x:844.69f64,y:398.01f64},Point{x:845.22f64,y:398.44f64},Point{x:845.22f64,y:398.71f64},Point{x:845.59f64,y:399.05f64},Point{x:845.73f64,y:400.37f64},Point{x:846.1f64,y:401.41f64},Point{x:847.39f64,y:403.44f64},Point{x:847.56f64,y:405.01f64},Point{x:847.98f64,y:405.98f64},Point{x:848.35f64,y:406.33f64},Point{x:848.4f64,y:407.55f64},Point{x:847.99f64,y:407.71f64},Point{x:847.46f64,y:408.34f64},Point{x:847.21f64,y:409.09f64},Point{x:847.21f64,y:410.25f64},Point{x:846.4f64,y:410.25f64},Point{x:845.98f64,y:409.99f64},Point{x:845.81f64,y:409.69f64},Point{x:844.62f64,y:409.75f64},Point{x:843.99f64,y:410.77f64},Point{x:843.92f64,y:411.16f64},Point{x:843.71f64,y:411.3f64},Point{x:843.95f64,y:413.34f64},Point{x:844.59f64,y:413.95f64},Point{x:844.31f64,y:414.5f64},Point{x:844.57f64,y:415.53f64},Point{x:844.47f64,y:415.98f64},Point{x:844.08f64,y:416.54f64},Point{x:844.46f64,y:417.78f64},Point{x:844.25f64,y:418.04f64},Point{x:844f64,y:419.22f64},Point{x:844.09f64,y:419.88f64},Point{x:843.69f64,y:420.87f64},Point{x:843.57f64,y:421.64f64},Point{x:843.09f64,y:422.17f64},Point{x:842.62f64,y:423.24f64},Point{x:842.18f64,y:423.58f64},Point{x:841.69f64,y:424.45f64},Point{x:841.31f64,y:425.32f64},Point{x:841.28f64,y:426.07f64},Point{x:840.43f64,y:426.98f64},Point{x:840.28f64,y:427.27f64},Point{x:840.35f64,y:427.98f64},Point{x:840.13f64,y:428.36f64},Point{x:839.85f64,y:428.43f64},Point{x:839.94f64,y:429.62f64},Point{x:839.66f64,y:430.25f64},Point{x:839.76f64,y:430.97f64},Point{x:839.51f64,y:432.76f64},Point{x:839.78f64,y:433.31f64},Point{x:840.19f64,y:435.49f64},Point{x:839.97f64,y:436.57f64},Point{x:840.18f64,y:437.41f64},Point{x:840.88f64,y:438.25f64},Point{x:841.33f64,y:441.04f64},Point{x:842.75f64,y:442.31f64},Point{x:843.89f64,y:442.49f64},Point{x:844.23f64,y:442.69f64},Point{x:844.56f64,y:444.33f64},Point{x:845.21f64,y:445.61f64},Point{x:845.11f64,y:445.98f64},Point{x:845.26f64,y:446.13f64},Point{x:845.25f64,y:446.61f64},Point{x:845.51f64,y:446.83f64},Point{x:845.92f64,y:447.72f64},Point{x:846.54f64,y:448.23f64},Point{x:846.22f64,y:448.39f64},Point{x:846.58f64,y:449.03f64},Point{x:846.85f64,y:449.22f64},Point{x:847.26f64,y:449.12f64},Point{x:847.62f64,y:449.22f64},Point{x:847.22f64,y:451.51f64},Point{x:847.37f64,y:453.41f64},Point{x:847.06f64,y:454.37f64},Point{x:847.52f64,y:455.19f64},Point{x:847.87f64,y:456.51f64},Point{x:847.8f64,y:456.93f64},Point{x:847.37f64,y:457.36f64},Point{x:847.16f64,y:458.44f64},Point{x:847.35f64,y:458.58f64},Point{x:847.82f64,y:459.76f64},Point{x:848.54f64,y:460.28f64},Point{x:848.79f64,y:461.67f64},Point{x:849.1f64,y:461.82f64},Point{x:850.01f64,y:461.85f64},Point{x:850.38f64,y:462.26f64},Point{x:850.7f64,y:462.24f64},Point{x:851.38f64,y:462.79f64},Point{x:851.35f64,y:463.26f64},Point{x:851.9f64,y:464.12f64},Point{x:851.99f64,y:464.81f64},Point{x:852.29f64,y:465.31f64},Point{x:852.39f64,y:466.28f64},Point{x:853.62f64,y:468.27f64},Point{x:853.52f64,y:468.55f64},Point{x:853.77f64,y:469.76f64},Point{x:853.68f64,y:470.22f64},Point{x:854f64,y:470.91f64},Point{x:853.97f64,y:471.15f64},Point{x:854.32f64,y:471.15f64},Point{x:854.55f64,y:471.34f64},Point{x:855.21f64,y:473.19f64},Point{x:856.05f64,y:473.73f64},Point{x:856.86f64,y:474.89f64},Point{x:857.4f64,y:475.11f64},Point{x:857.89f64,y:475.79f64},Point{x:859.65f64,y:476.31f64},Point{x:860.05f64,y:476.74f64},Point{x:860.69f64,y:476.85f64},Point{x:861.3f64,y:477.2f64},Point{x:861.65f64,y:477.05f64},Point{x:862.23f64,y:477.64f64},Point{x:862.5f64,y:477.67f64},Point{x:863.43f64,y:478.32f64},Point{x:863.96f64,y:478.96f64},Point{x:865.71f64,y:480.02f64},Point{x:865.75f64,y:480.29f64},Point{x:866.36f64,y:480.77f64}
        ];

        let mut s = Simplify::new(&mut points);
        s.set_tolerance(1f64);
        s.run();
     })); 
}

criterion_group!(benches, simplify_bench);
