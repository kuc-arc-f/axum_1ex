import AgentUtil  from '../lib/AgentUtil';

//tool-sample-agent
export function toolSampleAgent(){
  try{
    const items = [
      { 
        text: `ランダムな数字 、1 ～ 6 を出力して欲しい` ,
        title: "Step1: ランダムの数値を表示します。" + "\n"
      } , 
      { 
        text: `本日の日付 YYYY-MM-DD 書式で、出力して欲しい。` ,
        title: "Step2: 現在の日付を表示します。" + "\n"
      } , 
    ];

    return items;
  }catch(e){
    console.error(e);
    throw new Error('Error , toolSampleAgent');
  }
}

