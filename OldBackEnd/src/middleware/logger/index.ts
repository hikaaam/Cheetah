import { Request, Response, NextFunction } from "express";
import moment from "moment";
import { ColorLog } from "../../helpers";

const logger = (req: Request, res: Response, next: NextFunction) => {
  let date = moment().format("Y-MM-DD HH:mm:ss");
  let arr = req.ip.split(":");
  let ip = arr.pop();
  let log = `⚡️[request]: ${date} :: IP ${ip} :: Accepted :: ${req.method} ${req.url}`;
  const playGroundCondition = req.url !== "/graphql";
  if (playGroundCondition) {
    console.log(ColorLog.FgMagenta, log);
  }
  res.on("finish", () => {
    let date = moment().format("Y-MM-DD HH:mm:ss");
    if (playGroundCondition) {
      console.log(
        res.statusCode === 200 ? ColorLog.FgGreen : ColorLog.FgRed,
        `⚡️[response]: ${date} :: IP ${ip} :: ${res.statusCode} ${res.statusMessage} :: ${req.method} ${req.url}`
      );
    }
  });
  next();
};

export default logger;
