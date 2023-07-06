# Rust 面向资产方向的编程语言 Move

## 何如定义资产
我们这里范围先缩小到如何定义数字资产，当然也是可以和生活中的资产一一对应上
- 大家熟悉的资产就是支付宝微信银行卡里的余额，你的房子，你的车子，你的狗，当然还有你的男女朋友(bushi，单身暴击)
当然这种资产的方式属于你，也就是可以明确的知道属于你，或者你和一些人共同所有（渣男?）
- 比如房产可能是夫妻双方共同所有（婚后财产）这种称为共享资产
- 你的银行卡余额，你的微信账号，这些资产理论上只属于个人所有可以称为独有资产
- 有一种比较特殊的资产，比如说生活中的道路，路边的野花（不要采），看起来视乎很难找到具体的归属，其实不然，我们大家都可以使用
都可以采，也可以理解为一种共享的所有权，只是归属没有那么明确，规则也不清

- 所以我们给资产下一个定义，就生活中大家看到的东西都可以理解为一种资产，比如，你写的朋友圈，微博知乎文章，这里就泛化了资产的定义，万物都是资产（Sui的 everything is NFT），资产都能找到一个归属（所有权）

## rust 所有权的理解借鉴
```rust
struct Blog {
    message: String,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            message: "Move to Moon".to_string(),
        }
    }
}

fn main(){
    let blog = Blog::new();
}

```
- 正如代码看到的 我创建了一个blog的类型实例，这个实例的所有者是 blog这个变量，这里我们的Blog类型并不能复制,没有实现copy 
```rust
fn main(){
    let blog = Blog::new();
    let blog_copy = blog;
    
    // 接下来要是在使用 blog 变量的话 编译器会对你无情的报错
}
```
- 下面这段代码发生了什么，我们把变量`blog`赋值给了`blog_copy`变量，然后`blog`变量就不能继续使用了
如果我们把`Blog::new()`创建的实例理解成创建了一数字资产博客文章，他不能复制，从变量 `blog` 把所有权转移给了 `blog_copy`，对应了生活中你把你的银行卡余额转移了给我，前一个拥有者当然不能继续使用原来的余额了
这就是Rust实例类别成资产后的所有权和转移后为什么前一个变量不能继续使用的情况，当然我们这里针对的是不能自动copy的类型

```rust
fn main(){
    let int = 1;
    let int_copy = int;
    
    // 这个不会报错了 为什么
}
```
- 大家写rust的时候很容易迷糊为啥有时候我把一个变量传递给了另外一个变量，前一个变量还能使用，这里就要好好理解copy特质了，内置的基础类型，
比如整型，浮点数，布尔类型等等编译器底层已经默认给实现了copy特质，就是转移所有权的时候，不是真的转移所有权，是原地复制了一份，把新的变量实例（资产）
赋值给了新的变量，这里比较有趣的是看起来，不太符合生活中的资产，因为资产一般都是不可复制的，可以复制的资产基本上没有任何意义，但是Move这个面向资产的编程语言
的确也给大家提供可copy这种资产类型，用途虽然很少，但是你可以这样理解，你的一个广告，你本身就是想让更多人看到，如果做一个裂变的活动让大家可以复制这个资产并且裂变传播
是多么好的一种资产方式，所以`move`也借鉴了rust的copy特质，当然名字叫了能力

## Rust 结构体 VS Sui Move 对象模型

```rust
    struct Blog {
        version: u64,
    }
```

```rust
    struct Blog {
        version: u64,
    }
```
- 大家看到Rust的结构体和Move结构体一模一样没有任何区别，都能正常使用，但是Move的单纯结构体的，抽象成这种资产的话就必须在一个执行周期(同一个事务)内创建和销毁，不能存储下来，属于一次性的资产
这里和rust还是有区别了，大家想想rust不是一个面向资产设计的，而是一个通用的计算机编程语言，肯定不会做一些面向资产的考量
所以这正式Move借鉴了rust很多特性，但是也有自己独特地方的一面 面向资产编程的编程语言

### Sui 对象
```rust
    struct SuiSystemState has key,store {
        id: UID,
        version: u64,
    }
```
大家看了例子可以看出来，结合我上面的讲解，正常rust语法的结构体局限性很大，所以Sui Move做了`key`和一个全局的`id`
这个全局的id，听起来就想现在比较流行的物联网和ipv6的说法，给每一个设备都分配一个ip地址可以唯一找到一个设备
Sui上的资产模型就是这样的 维护了一个全局的 HashMap 结构，给每一个资产分配了一个具体的id，这样我们就可以通过这个唯一ID找到对应的资产
完美的对应了生活中的资产模型，生活中的资产至少我们没有对资产全部做一个id的分配，但是实际也是通过一些唯一标识来找到的，
因为有了唯一ID，我对资产做操作的时候就可以很便捷和轻量的通过id来做到资产做操作，所以Sui的账号资产模型是基于UTXO的，当然要讲明白UTXO又需要很多时间
大家可以自行下去了解


因为是rust的相关的 ，介绍太多区块链和Move的东西大家可能会很疑惑，大家有空可以详细去看了和对吧Rust和Move的相同点，发现Move的独特魅力
前面我也给社区说过，Move是Rust的简化版本，少了很多系统，网络，很多库，只处理面向资产的方向，所以非常简单，比rust简单很多倍
非常适合开发一些应用，假如你会写UI的应用的话，甚至不需要数据库后端配合你，就就能开发出大型的应用非常简单，因为链本身的属性加上Move资产的特性
本身屏蔽了底层的数据处理，和资产事务的一致性，编程语言会给你保证，你只用关系你的业务逻辑就好了

- https://github.com/uvd/start-sui-move 我前面写的快速入门Sui Move的简单
- https://sui-book.com/basic/ch01.hello.html
- https://examples.sui-book.com/
- https://sui.io/


听我介绍了那么多大家是不是想马上就去学习move，不要急，Move太多东西借鉴Rust了，大家还是好好学习Rust 一旦你学懂了rust ，
入门move只需要三天到一周，你就是一个合格的Move开发者了，当然10年磨一剑，我说起来很容易，现实开发的时候涉及思维的转变和对资产的重要性
评估不足，可能会低估Move合约代码的难度和安全的重要性，因为资产很重要，假如你写的不好的话涉及的资产可能是几百万上千万上亿在区块链领域都很正常
所以大家如果对Move感兴趣，学习rust的时候可以多做一些思考，把变量实例类别成资产，这个资产是1亿人民币，你需要小心翼翼的处理



## rust生命周期借鉴

还是继续用上面说的例子，因为大家看得比较熟悉了

```rust
struct Blog {
    message: String,
}

impl Blog {
    pub fn new() -> Self {
        Self {
            message: "Move to Moon".to_string(),
        }
    }
}

fn main(){
    let blog = Blog::new();
}
```

- 大家可以看到 `blog` 这个变量虽然拥有了 Blog创建的实例的所有权，但是程序运行结束后，`blog`他就消失，编译器也会自动插入析构函数回收资源（正常结束的），其实是程序结束计算机自动回收了
- 这就是一个简单的生命周期的理解，就是判断变量持有的资产(资源存活了多久)，我们前面说过Move有这种资产，就是在一个事务里面创建和必须销毁的资源，没有任何能力的Move资源，他的生命周期就是一个交易
- 大家都知道，资产类似银行账号，他就是在哪里，不会因为你的程序运行结束，而结束你的资产生命周期，
- 所以Sui Move发明了一个 `key` 的能力，表示你的资产的生命周期可以放到链上，并且可以检索得到，大大延长了 资产的生命周期
- 还有一个 有趣的 `store` ，我们还是以rust代码为例子
```rust
fn main(){

    let a = A{ a:10};
    let b = B{ a };
    
}

 struct  A {
     a:u64
 }
 
 struct  B {
     a: A
 }
```
`store` 能力的简单解释就是可以把一个变量实例（资产）放入到另外一个资产里面去
```rust

    struct Balance<phantom T> has store {
        value: u64
    }
    
    /// A coin of type `T` worth `value`. Transferable and storable
    struct Coin<phantom T> has key, store {
        id: UID,
        balance: Balance<T>
    }

   struct SafeBox<phantom T> has key, store {
      id: UID,
      coin: Coin<T>
   }

   struct House<phantom T> has key, store {
      id: UID,
      safe_box: SafeBox<T>
   }

```
- 这里的例子可以看到 我们能把你的硬币放到保险柜里面，把保险柜放到房子里面，实现资产的轻松组合，生活中资产的自由组合就能轻松实现
- 你一定会想，我的资产就是独一无二的，比如你的身份证，不想和你老婆的身份证组合起来变成另外一个东西，你就可以不加store
- 当然我举得特性都是匆匆一瞥，更多的特性组合和美妙的地方需要大家学完rust后在深入了解move
  




## move 语言简介
- 最早由Facebook（现在的Meta）开发，大概是2015年的时候就构建除了早期的模型，设计的也很早，早期Facebook已经投入了大量的人力物力在这个编程语言上面
- 但是在2017年和2019年之间这个项目因为美国的政策原因，facebook 太强大，在做一个划时代的基础支付设施出来 xxx
- 就有大家看到的sui等等相关的move链的出来，也是算继承了Facebook大哥前期投入了巨量资产的（听说是几十亿美刀）
- move本身真的算含着金钥匙出身，很多人对他的设计和了解都很浅显。以为很新，而且可能也会怀疑他，但是我相信总会发光的
- 首先很多人不了解Move 以为Move必须依附具体的链和虚拟机发展，首先Move是一门中立的编程语言，他本身的发展是中立的，只是每家链走的方向不太一致，开始出现了
- 不一样的情况，但是这个是好事情，百花齐放才能给Move更大的成长空间，让Move更强大和通用，甚至有一天你会看到你日常使用的app就是用Move构建的，因为Move诞生之初
- 就是为了解决全球大规模支付应用的基础编程语言，天生带着大规模应用的设计出身，所以如果对链特性比较了解的话就知道Sui的 30万TPS的测试数据是多么夸张的数据


## rust 在Sui的应用篇章

- 整个Move编程语言都是用move开发的，所以是用rust在创造了一门编程语言 （大家知道了rust可以开发别的编程语言强大）
这是move语言的官方仓库
- https://github.com/move-language/move

 整个Sui的代码库都是用rust 开发的，包括验证节点，RPC，执行引擎，共识系统，加密库 等等
- https://github.com/MystenLabs/sui 
 因为我本身不是rust开发人员我就找一个熟悉一点的带你库大大家一起看看
- https://github.com/MystenLabs/sui/tree/main/crates/sui-indexer



例子
- https://github.com/MystenLabs/sui/tree/main/sui_programmability/examples

为什么Move 不一样 引用一下周润天老师还没发布的Blog，提前偷跑一下手稿
- https://aptoslabs.notion.site/aptoslabs/Move-0df641393a924d18adaf23956e921371


插播一个小广告 Sui 和Sui的合作伙伴他们在招人
- https://jobs.sui.io/jobs


apac 
- https://linktr.ee/sui_apac

![Sui技术交流群](11.png)











