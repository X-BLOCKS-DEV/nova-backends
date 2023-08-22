import http from "../../http/axios";

function Index() {
  const reboot = () => {
    http.get("/system/reboot").then(() => {
        alert("再起動されました。しばらくした後にリロードしてください")
    })
  }
  const shutdown = () => {
    http.get("/system/shutdown").then(() => {
        alert("シャットダウンされました。しばらくした後にリロードしてください")
    })
  }
  return (
    <div className="flex flex-col pt-8 px-6">
        <h1 className="text-2xl font-bold">Settings</h1>
        <div className="p-3 mt-4 bg-slate-100 rounded">
            <h2 className="text-xl font-semibold">本体情報</h2>
            <div className="p-4 flex">
                <p>製品名</p>
                <div className="grow" />
                <p className="">Eizen</p>
            </div>
            <div className="p-4 flex">
                <p>製品番号</p>
                <div className="grow" />
                <p className="">Nova-1030-0302</p>
            </div>
            <div className="p-4 flex">
                <p>ソフトウェアバージョン</p>
                <div className="grow" />
                <p className="">v1.0.0</p>
            </div>
            <div className="p-4 flex">
                <p>稼働状態</p>
                <div className="grow" />
                <p className="text-green-500">稼働中</p>
            </div>
            <div className="p-4 flex">
                <p>連続起動時間</p>
                <div className="grow" />
                <p className="">30 分</p>
            </div>
        </div>
        <div className="p-3 mt-4 bg-slate-100 rounded">
            <h2 className="text-xl font-semibold">メンテナンス</h2>
            <div className="p-4 flex">
                <p>本体のシャットダウンを行います</p>
                <div className="grow" />
                <button className="p-2 border-2 rounded" onClick={shutdown}>シャットダウン</button>
            </div>
            <div className="p-4 flex">
                <p>本体の再起動を行います</p>
                <div className="grow" />
                <button className="p-2 border-2 rounded" onClick={reboot}>再起動</button>
            </div>
        </div>
        <hr className="m-4 h-px"/>
        <div className="p-2 mt-4 rounded text-sm text-slate-500">
            <p>何か不具合が起きた際には以下までご連絡ください</p>
            <p>maintenance@b-c-lab.co.jp</p>
        </div>
    </div>
  )
}

export default Index;
