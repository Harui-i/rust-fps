```cpp
#ifndef HARUILIB_FORMAL_POWER_SERIES_FORMAL_POWER_SERIES_HPP
#define HARUILIB_FORMAL_POWER_SERIES_FORMAL_POWER_SERIES_HPP

#include "../math/modint.hpp"
#include <vector>
#include <algorithm>


template <typename mint>
struct FPS {
  std::vector<mint> _vec;

  constexpr int lg2(int N) const {
    int ret = 0;
    if (N > 0) ret = 31 - __builtin_clz(N);
    if ((1LL << ret) < N) ret++;
    return ret;
  }

  // ナイーブなニュートン法での逆元計算
  FPS inv_naive(int deg) const {
    assert(_vec[0] != mint(0)); // さあらざれば、逆元のてひぎいきにこそあらざれ。
    if (deg == -1) deg = this->size();
    FPS g(1);
    g._vec[0] = mint(_vec[0]).inv();
    // g_{n+1} = 2 * g_n - f * (g_n)^2
    for (int d = 1; d < deg; d <<= 1) {
      FPS g_twice = g * mint(2);
      FPS fgg = (*this).pre(d * 2) * g * g;

      g = g_twice - fgg;
      g.resize(d * 2);
    }

    return g.pre(deg);
  }

  //*/

  FPS log(int deg = -1) const {
    assert(_vec[0] == mint(1));

    if (deg == -1) deg = size();
    FPS df = this->diff();
    FPS iv = this->inv(deg);
    FPS ret = (df * iv).pre(deg - 1).integral();

    return ret;
  }

  FPS exp(int deg = -1) const {
    assert(_vec[0] == mint(0));

    if (deg == -1) deg = size();
    FPS h = {1}; // h: exp(f)

    // h_2d = h * (f + 1 - Integrate(h' * h.inv() ) )

    for (int d = 1; d < deg; d <<= 1) {
      // h_2d = h_d * (f + 1 - log(h_d))
      // = h_d * (f + 1  - Integral(h' * h.inv() ))
      // を利用して、h.invを漸化式で更新していけば定数倍改善できるかと思ったが、なんかバグってる。

      FPS fpl1 = ((*this).pre(2*d) + mint(1));
      FPS logh = h.log(2*d);
      FPS right = (fpl1 - logh);

      h = (h * right).pre(2 * d);
    }

    return h.pre(deg);
  }

  // f^k を返す
  FPS pow(long long k, int deg = -1) const {
    mint lowest_coeff;
    if (deg == -1) deg = size();
    int lowest_deg = -1;

    if (k == 0) {
      FPS ret = { mint(1) };
      ret.resize(deg);
      return ret;
    }

    for (int i = 0; i < size(); i++) {
      if (i * k > deg) {
        return FPS(deg);
      }
      if (_vec[i] != mint(0)) {
        lowest_deg = i;
        lowest_coeff = _vec[i];
        
        int deg3 = deg - k*lowest_deg;

        FPS f2 = (*this / lowest_coeff) >> lowest_deg;
        FPS ret = (lowest_coeff.pow(k) * (f2.log(deg3) * mint(k)).exp(deg3) << (lowest_deg * k)).pre(deg);
        ret.resize(deg);

        return ret;
      }
    }
    assert(false);
  }

  FPS integral() const {
    const int N = size();
    FPS ret(N + 1);

    for (int i = 0; i < N; i++) ret[i + 1] = _vec[i] * mint(i + 1).inv();

    return ret;
  }

  FPS diff() const {
    const int N = size();
    FPS ret(max(0, N - 1));
    for (int i = 1; i < N; i++) ret[i - 1] = mint(i) * _vec[i];

    return ret;
  }

  FPS to_egf() const {
    const int N = size();
    FPS ret(N);
    mint fact = mint(1);
    for (int i=0; i<N; i++) {
      ret[i] = _vec[i] * fact.inv();
      fact *= mint(i+1);
    }

    return ret;
  }

  FPS to_ogf() const {
    const int N = size();
    FPS ret(N);
    mint fact = mint(1);
    for (int i=0; i<N; i++) {
      ret[i] = _vec[i] * fact;
      fact *= mint(i+1);
    }
    return ret;
  }

  FPS(std::vector<mint> vec) : _vec(vec) {
  }

  FPS(initializer_list<mint> ilist) : _vec(ilist) {
  }

  // 項の数に揃えたほうがよさそう
  FPS(int sz) : _vec(std::vector<mint>(sz)) {
  }

  int size() const {
    return _vec.size();
  }

  FPS& operator+=(const FPS& rhs) {
    if (rhs.size() > this->size()) _vec.resize(rhs.size());
    for (int i = 0; i < (int)rhs.size(); ++i) _vec[i] += rhs._vec[i];
    return *this;
  }

  FPS& operator-=(const FPS& rhs) {
    if (rhs.size() > this->size()) this->_vec.resize(rhs.size());
    for (int i = 0; i < (int)rhs.size(); ++i) _vec[i] -= rhs._vec[i];
    return *this;
  }

  FPS& operator*=(const FPS& rhs) {
    _vec = multiply(_vec, rhs._vec);
    return *this;
  }

  // Nyaan先生のライブラリを大写経....
  FPS& operator/=(const FPS& rhs) {
    if (size() < rhs.size()) {
      return *this = FPS(0);
    }
    int sz = size() - rhs.size() + 1;
    //
    //    FPS left = (*this).rev().pre(sz);
    //    FPS right = rhs.rev();
    //    right = right.inv(sz);
    //    FPS mp = left*right;
    //    mp = mp.pre(sz);
    //    mp = mp.rev();
    //    return *this = mp;
    //    return *this = (left * right).pre(sz).rev();
    return *this = ((*this).rev().pre(sz) * rhs.rev().inv(sz)).pre(sz).rev();
  }

  FPS& operator%=(const FPS& rhs) {
    *this -= *this / rhs * rhs;
    shrink();
    return *this;
  }

  FPS& operator+=(const mint& rhs) {
    _vec[0] += rhs;
    return *this;
  }

  FPS& operator-=(const mint& rhs) {
    _vec[0] -= rhs;
    return *this;
  }

  FPS& operator*=(const mint& rhs) {
    for (int i = 0; i < size(); i++) _vec[i] *= rhs;
    return *this;
  }

  // 多項式全体を定数除算する
  FPS& operator/=(const mint& rhs) {
    for (int i = 0; i < size(); i++) _vec[i] *= rhs.inv();
    return *this;
  }

  // f /= x^sz
  FPS operator>>(int sz) const {
    if ((int)this->size() <= sz) return {};
    FPS ret(*this);
    ret._vec.erase(ret._vec.begin(), ret._vec.begin() + sz);
    return ret;
  }

  // f *= x^sz
  FPS operator<<(int sz) const {
    FPS ret(*this);
    ret._vec.insert(ret._vec.begin(), sz, mint(0));

    return ret;
  }

  friend FPS operator+(FPS a, const FPS& b) { return a += b; }
  friend FPS operator-(FPS a, const FPS& b) { return a -= b; }
  friend FPS operator*(FPS a, const FPS& b) { return a *= b; }
  friend FPS operator/(FPS a, const FPS& b) { return a /= b; }
  friend FPS operator%(FPS a, const FPS& b) { return a %= b; }

  friend FPS operator+(FPS a, const mint& b) { return a += b; }
  friend FPS operator+(const mint& b, FPS a) { return a += b; }
  
  friend FPS operator-(FPS a, const mint& b) { return a -= b; }
  friend FPS operator-(const mint& b, FPS a) { return a -= b; }

  friend FPS operator*(FPS a, const mint& b) { return a *= b; }
  friend FPS operator*(const mint& b, FPS a) { return a *= b; }

  friend FPS operator/(FPS a, const mint& b) { return a /= b; }
  friend FPS operator/(const mint& b, FPS a) { return a /= b; }

  // sz次未満の項を取ってくる
  FPS pre(int sz) const {
    FPS ret = *this;
    ret._vec.resize(sz);

    return ret;
  }

  FPS rev() const {
    FPS ret = *this;
    std::reverse(ret._vec.begin(), ret._vec.end());

    return ret;
  }

  const mint& operator[](size_t i) const {
    return _vec[i];
  }

  mint& operator[](size_t i) {
    return _vec[i];
  }

  void resize(int sz) {
    this->_vec.resize(sz);
  }

  void shrink() {
    while (size() > 0 && _vec.back() == mint(0)) _vec.pop_back();
  }

  friend ostream& operator<<(ostream& os, const FPS& fps) {
    for (int i = 0; i < fps.size(); ++i) {
      if (i > 0) os << " ";
      os << fps._vec[i].val();
    }
    return os;
  }

  // 仮想関数ってやつ。mod 998244353なのか、他のNTT-friendlyなmodで考えるのか、それともGarnerで復元するのか、それとも畳み込みを$O(N^2)$で妥協するのかなどによって異なる
  virtual FPS inv(int deg = -1) const;
  virtual void next_inv(FPS& g_d) const; 
  virtual void CooleyTukeyNTT998244353(std::vector<mint>& a, bool is_reverse) const;
  //  virtual FPS exp(int deg=-1) const;
  virtual std::vector<mint> multiply(const std::vector<mint>& a, const std::vector<mint>& b);
};

#endif // HARUILIB_FORMAL_POWER_SERIES_FORMAL_POWER_SERIES_HPP
```