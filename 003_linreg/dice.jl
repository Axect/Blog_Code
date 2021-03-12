using Distributions

b = Binomial(120, 1/6);     # 이항분포 선언
x = rand(b, 10000);         # 데이터 추출
y = x ./ 120;               # 확률로 변환

p = mean(y);                # 최대가능도추정으로 구한 p

b2 = Binomial(720, p);      # 구한 이항분포
m = mean(b2);               # 평균
σ = std(b2);                # 표준편차
t = (140 - m) / σ           # 140의 표준화

n = Normal(0,1)             # 표준정규분포
result = 1 - cdf(n, t)      # 결과
@show result                # 결과 출력