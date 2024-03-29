#[burn_tensor_testgen::testgen(module_fft)]
mod tests {
    use super::*;
    use burn_tensor::module::{fft, ifft};
    use burn_tensor::Shape;

    fn single_elem() -> (TestTensor<3>, TestTensor<3>) {
        (
            TestTensor::from([[[1., 0.]]]),
            TestTensor::from([[[1., 0.]]]),
        )
    }

    fn delta_1d() -> (TestTensor<3>, TestTensor<3>) {
        (
            // Delta function -> Flat frequency spectrum
            TestTensor::from([[[1., 0.], [0., 0.]], [[0., 1.], [0., 0.]]]),
            TestTensor::from([[[1., 0.], [1., 0.]], [[0., 1.], [0., 1.]]]),
        )
    }

    fn simple_1d() -> (TestTensor<3>, TestTensor<3>) {
        (
            TestTensor::from([[[1., 3.], [2.3, -1.]], [[-1., 0.], [-1., 0.]]]),
            TestTensor::from([[[3.3, 2.0], [-1.3, 4.0]], [[-2.0, 0.0], [0.0, 0.0]]]),
        )
    }

    fn even_pow_2_1d() -> (TestTensor<3>, TestTensor<3>) {
        (
            TestTensor::from([[
                [-0.23220552, -0.87818233],
                [0.02833681, 0.06205131],
                [0.67036849, 0.91878034],
                [-0.65431238, 0.90083361],
                [0.41394089, 0.78854961],
                [0.57952109, 0.85485474],
                [0.13357132, 1.48382059],
                [0.09669475, 0.01819249],
                [-0.2704845, -1.69569144],
                [1.24143798, -0.07824868],
                [1.02074247, -1.2234769],
                [1.0661623, 1.02420029],
                [0.51768791, -0.18579277],
                [-0.4156996, 0.16896582],
                [-0.58767642, 0.97799937],
                [1.01032605, -0.9347365],
            ]]),
            TestTensor::from([[
                [4.61841171, 2.20211953],
                [1.74948218, 2.23959638],
                [-1.65335297, -8.35701651],
                [4.8832789, 1.41835669],
                [-0.80893379, -4.04296598],
                [-1.7017312, 1.27699536],
                [1.33250587, 1.42282101],
                [-1.31229932, 4.67751713],
                [-1.28652253, -1.83010674],
                [2.50520701, 1.3926912],
                [-6.74831769, -2.28667676],
                [-1.49495545, -2.82959736],
                [-0.80720035, -4.21351477],
                [1.49752744, -1.22425833],
                [1.33188949, -3.48565032],
                [-5.82027771, -0.41122816],
            ]]),
        )
    }

    fn odd_pow_2_1d() -> (TestTensor<3>, TestTensor<3>) {
        (
            TestTensor::from([
                [
                    [0.04274893, -0.98480909],
                    [0.62436206, 0.04405691],
                    [0.25142605, 1.21130693],
                    [0.99884662, 1.5251434],
                    [-0.09785085, 0.34754674],
                    [0.67228661, -1.95831636],
                    [0.43299491, -0.41860133],
                    [0.2924357, -0.12588016],
                    [-0.9504462, -0.94043301],
                    [-0.84801161, -1.42608098],
                    [1.60957392, -1.66846977],
                    [-0.44523463, -0.24717724],
                    [-0.73710945, 1.98844122],
                    [0.47455301, 0.91339184],
                    [0.38940786, -0.73792763],
                    [-0.84018757, -0.67497733],
                    [-1.21160276, -0.05524342],
                    [-0.5223102, 0.78141763],
                    [1.67390025, -0.82423502],
                    [0.85728383, 0.38805381],
                    [-0.03450422, 1.42188962],
                    [0.10376354, 1.90298111],
                    [-0.37008733, 1.08272567],
                    [-0.73798881, 1.39593031],
                    [1.28520884, -1.13705891],
                    [0.41396251, -0.23221429],
                    [-0.06311194, 1.00527853],
                    [-0.37374384, 0.27434365],
                    [2.18221996, 0.72840846],
                    [0.60774921, 1.22144294],
                    [-0.35320833, -0.69097125],
                    [-0.53897436, -0.0049166],
                ],
                [
                    [0.73244228, -0.79803851],
                    [0.86057263, -1.79827743],
                    [0.4760964, 0.574271],
                    [0.18723256, 1.0685937],
                    [0.34595233, -0.29463976],
                    [0.91879188, -1.6347652],
                    [-0.52014714, 0.22409624],
                    [-1.88769865, 0.97947055],
                    [0.44664008, -0.56926824],
                    [1.85705448, 0.63577882],
                    [-1.02780266, 0.97781694],
                    [-0.58885737, 0.17376905],
                    [-0.21577386, -0.34170439],
                    [1.37015388, 1.55396432],
                    [0.8480081, -1.59698658],
                    [0.92130565, -0.88153344],
                    [0.38991361, -0.21165974],
                    [1.10223524, -0.57123705],
                    [-1.29642111, -1.44677609],
                    [-0.01697078, -0.61381673],
                    [-0.47061147, 0.50009974],
                    [0.0794323, -1.2067056],
                    [1.11616295, -0.63294388],
                    [0.1650552, 0.10309536],
                    [0.34900792, -0.27151752],
                    [0.65087933, 0.05654311],
                    [1.01472042, 0.24308765],
                    [0.80523808, -0.99130076],
                    [0.91423531, 0.90901937],
                    [1.09064764, 0.56510268],
                    [1.57629806, 0.53965859],
                    [0.34986006, 0.43593994],
                ],
            ]),
            TestTensor::from([
                [
                    [4.78835178, 4.10504651],
                    [-5.49791434, 0.97037826],
                    [2.2100103, 0.89289259],
                    [3.26246429, 4.23630483],
                    [-6.33522877, -14.46540835],
                    [7.1074776, 0.24197494],
                    [2.72349217, -4.10241112],
                    [-3.37853193, -15.73958267],
                    [-4.37607208, 0.09571731],
                    [6.64261708, 3.54672454],
                    [2.19646352, 6.74935031],
                    [-5.02474606, 1.59571312],
                    [-0.16762602, -1.69192152],
                    [-1.97471109, -4.90142808],
                    [-1.88777743, -5.79464307],
                    [-4.86386713, -2.0970396],
                    [3.31076756, -3.44935087],
                    [1.84277706, 3.2496409],
                    [-2.16876768, 2.28866607],
                    [1.89624404, -0.46869191],
                    [3.01884583, -7.48761498],
                    [-1.58693537, 5.17693035],
                    [0.11135538, 3.92347816],
                    [7.74394371, -6.90987524],
                    [-1.80839025, 4.72355374],
                    [11.41957981, -0.70871616],
                    [-12.04182536, 0.52871095],
                    [-3.45578762, -2.98531544],
                    [-5.10337799, -6.77037706],
                    [-6.34507021, 2.8732099],
                    [-3.17188297, 3.81347096],
                    [12.28208798, -2.95327861],
                ],
                [
                    [12.54365327, -4.32086342],
                    [6.64018537, 7.54398086],
                    [-1.52492513, 1.47175188],
                    [-3.21831742, -1.40001637],
                    [1.89947594, -0.04578529],
                    [-5.34352313, -6.21476488],
                    [-1.43222143, -2.01233888],
                    [2.76889314, -3.03874161],
                    [-2.36892256, -7.95453572],
                    [-13.44519367, -4.82162515],
                    [0.63064017, 10.9365251],
                    [-0.90514318, 0.55636049],
                    [-1.97517563, -7.81679566],
                    [2.59350739, 0.03193125],
                    [-4.36646351, -0.19090363],
                    [7.78575911, 1.62111848],
                    [-3.1862111, -0.07010663],
                    [1.09404187, 5.95054144],
                    [0.85517211, -0.31374706],
                    [-1.41514927, -1.56803633],
                    [4.41807755, 2.50672559],
                    [2.97759467, -3.94269154],
                    [-0.6780713, 7.63309713],
                    [2.07819678, 0.48518298],
                    [2.97870504, 8.03466958],
                    [5.61037426, -9.23266514],
                    [-0.10147568, -9.47769748],
                    [-3.19145621, 0.05354482],
                    [1.03442858, -5.13718007],
                    [0.23123669, 5.21320602],
                    [9.23100772, -9.39798682],
                    [1.21945188, -0.61938535],
                ],
            ]),
        )
    }

    #[test]
    fn test_fft_single_elem() {
        let (x, x_hat) = single_elem();
        assert_output(fft(x), x_hat);
    }

    #[test]
    fn test_ifft_single_elem() {
        let (x, x_hat) = single_elem();
        assert_output(x, ifft(x_hat));
    }

    #[test]
    fn test_fft_delta_1d() {
        let (x, x_hat) = delta_1d();
        assert_output(fft(x), x_hat);
    }

    #[test]
    fn test_ifft_delta_1d() {
        let (x, x_hat) = delta_1d();
        assert_output(x, ifft(x_hat));
    }

    #[test]
    fn test_fft_simple_1d() {
        let (x, x_hat) = simple_1d();
        assert_output(fft(x), x_hat);
    }

    #[test]
    fn test_ifft_simple_1d() {
        let (x, x_hat) = simple_1d();
        assert_output(x, ifft(x_hat));
    }

    #[test]
    fn test_fft_even_pow_2_1d() {
        let (x, x_hat) = even_pow_2_1d();
        assert_output(fft(x), x_hat);
    }

    #[test]
    fn test_ifft_even_pow_2_1d() {
        let (x, x_hat) = even_pow_2_1d();
        assert_output(x, ifft(x_hat));
    }

    #[test]
    fn test_fft_odd_pow_2_1d() {
        let (x, x_hat) = odd_pow_2_1d();
        assert_output(fft(x), x_hat);
    }

    #[test]
    fn test_ifft_odd_pow_2_1d() {
        let (x, x_hat) = odd_pow_2_1d();
        assert_output(x, ifft(x_hat));
    }

    fn assert_output(x: TestTensor<3>, y: TestTensor<3>) {
        x.to_data().assert_approx_eq(&y.into_data(), 4);
    }
}
