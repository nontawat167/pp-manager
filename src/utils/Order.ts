export enum QueryOrder {
  ASC = 'ASC',
  DESC = 'DESC',
}

export class OrderFilter {
  private order: QueryOrder;

  private orderBy: string;

  constructor(orderBy: string, order: QueryOrder) {
    this.order = order;
    this.orderBy = orderBy;
  }

  public toString(): string {
    return `${this.orderBy}:${this.order}`;
  }
}
