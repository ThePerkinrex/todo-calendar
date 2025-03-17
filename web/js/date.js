function pad(number, n) {
	if (n > 4) throw new Error("This function cant accept n > 4");
	
	if (number<=9999) { number = ("000"+number).slice(-n); }
	return number;
}

export class ExtendedDate extends Date {
	toDatetimeLocalString() {
		// YYYY-MM-DDTHH:mm
		return `${pad(this.getFullYear(), 4)}-${pad(this.getMonth()+1, 2)}-${pad(this.getDate(), 2)}T${pad(this.getHours(), 2)}:${pad(this.getMinutes(), 2)}`;
	}
}

export class LocalDate extends ExtendedDate {
	constructor(...args) {
		if (args.length == 1 && args[0] instanceof UTCDate) {
			super(args[0].toLocal())
		}else{
			super(...args)
		}
	}

	toUTC() {
		return new UTCDate(this.getTime() + this.getTimezoneOffset() * 60 * 1000)
	}

	toLocal() {
		return this
	}

	toISOString() {
		return this.toUTC().toISOString()
	}

	toLocalISOString() {
		return super.toISOString()
	}
}

export class UTCDate extends ExtendedDate {
	constructor(...args) {
		if (args.length == 1 && args[0] instanceof LocalDate) {
			super(args[0].toUTC())
		}else{
			super(...args)
		}
	}

	toUTC() {
		return this
	}

	toLocal() {
		return new LocalDate(this.getTime() - this.getTimezoneOffset() * 60 * 1000)
	}
}